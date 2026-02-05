use crate::{
    auth::ApiKey,
    db::DbPool,
    models::{AnswerOption, CreateQuestion, UpdateQuestion, Question, QuestionWithOptions},
};
use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};

#[utoipa::path(
    get,
    path = "/questions/event/{id}",
    params(
        ("id" = i64, Path, description = "Event ID")
    ),
    responses(
        (status = 200, description = "Get questions for an event", body = Vec<QuestionWithOptions>)
    )
)]
pub async fn get_questions_by_event(
    State(pool): State<DbPool>,
    Path(event_id): Path<i64>,
) -> Json<Vec<QuestionWithOptions>> {
    // Fetch questions
    let questions = sqlx::query_as::<_, Question>("SELECT * FROM questions WHERE event_id = ?")
        .bind(event_id)
        .fetch_all(&pool)
        .await
        .unwrap_or_else(|_| vec![]);

    if questions.is_empty() {
        return Json(vec![]);
    }

    let _question_ids: Vec<i64> = questions.iter().map(|q| q.id).collect();

    let mut result = Vec::new();

    for q in questions {
        let options =
            sqlx::query_as::<_, AnswerOption>("SELECT * FROM options WHERE question_id = ?")
                .bind(q.id)
                .fetch_all(&pool)
                .await
                .unwrap_or_else(|_| vec![]);

        result.push(QuestionWithOptions {
            question: q,
            options,
        });
    }

    Json(result)
}

#[utoipa::path(
    get,
    path = "/questions/random",
    responses(
        (status = 200, description = "Get a random quiz", body = Vec<QuestionWithOptions>)
    )
)]
pub async fn get_random_quiz(State(pool): State<DbPool>) -> Json<Vec<QuestionWithOptions>> {
    // Get 5 random questions
    let questions =
        sqlx::query_as::<_, Question>("SELECT * FROM questions ORDER BY RANDOM() LIMIT 5")
            .fetch_all(&pool)
            .await
            .unwrap_or_else(|_| vec![]);

    let mut result = Vec::new();
    for q in questions {
        let options =
            sqlx::query_as::<_, AnswerOption>("SELECT * FROM options WHERE question_id = ?")
                .bind(q.id)
                .fetch_all(&pool)
                .await
                .unwrap_or_else(|_| vec![]);

        result.push(QuestionWithOptions {
            question: q,
            options,
        });
    }

    Json(result)
}

#[utoipa::path(
    post,
    path = "/questions",
    request_body = CreateQuestion,
    params(
        ("x-api-key" = String, Header, description = "API Key for authentication")
    ),
    responses(
        (status = 201, description = "Question created successfully", body = QuestionWithOptions),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal Server Error")
    )
)]
pub async fn create_question(
    State(pool): State<DbPool>,
    _api_key: ApiKey,
    Json(payload): Json<CreateQuestion>,
) -> impl IntoResponse {
    // Transaction for atomicity
    let mut tx = match pool.begin().await {
        Ok(tx) => tx,
        Err(e) => {
            tracing::error!("Failed to begin transaction: {:?}", e);
            return (StatusCode::INTERNAL_SERVER_ERROR, "Transaction failed").into_response();
        }
    };

    // 1. Insert Question
    let question_result = sqlx::query_as::<_, Question>(
        "INSERT INTO questions (event_id, question_text, explanation, difficulty_level) VALUES (?, ?, ?, ?) RETURNING *"
    )
    .bind(payload.event_id)
    .bind(payload.question_text)
    .bind(payload.explanation)
    .bind(payload.difficulty_level) // Default 'Medium' handled by DB if None, but we bind Option wrapper
    .fetch_one(&mut *tx)
    .await;

    let question = match question_result {
        Ok(q) => q,
        Err(e) => {
            tracing::error!("Failed to insert question: {:?}", e);
            let _ = tx.rollback().await;
            return (StatusCode::INTERNAL_SERVER_ERROR, "Failed to insert question").into_response();
        }
    };

    // 2. Insert Options
    let mut saved_options = Vec::new();
    for opt in payload.options {
        let opt_res = sqlx::query_as::<_, AnswerOption>(
            "INSERT INTO options (question_id, option_text, is_correct) VALUES (?, ?, ?) RETURNING *"
        )
        .bind(question.id)
        .bind(opt.option_text)
        .bind(opt.is_correct)
        .fetch_one(&mut *tx)
        .await;

        match opt_res {
            Ok(o) => saved_options.push(o),
            Err(e) => {
                tracing::error!("Failed to insert option: {:?}", e);
                let _ = tx.rollback().await;
                return (StatusCode::INTERNAL_SERVER_ERROR, "Failed to insert options").into_response();
            }
        }
    }

    // Commit
    if let Err(e) = tx.commit().await {
        tracing::error!("Failed to commit transaction: {:?}", e);
        return (StatusCode::INTERNAL_SERVER_ERROR, "Commit failed").into_response();
    }

    (
        StatusCode::CREATED,
        Json(QuestionWithOptions {
            question,
            options: saved_options,
        }),
    )
        .into_response()
}

#[utoipa::path(
    put,
    path = "/questions/{id}",
    request_body = UpdateQuestion,
    params(
        ("id" = i64, Path, description = "Question ID"),
        ("x-api-key" = String, Header, description = "API Key for authentication")
    ),
    responses(
        (status = 200, description = "Question updated successfully", body = Question),
        (status = 401, description = "Unauthorized"),
        (status = 404, description = "Question not found"),
        (status = 500, description = "Internal Server Error")
    )
)]
pub async fn update_question(
    State(pool): State<DbPool>,
    Path(id): Path<i64>,
    _api_key: ApiKey,
    Json(payload): Json<UpdateQuestion>,
) -> impl IntoResponse {
    let result = sqlx::query_as::<_, Question>(
        r#"
        UPDATE questions 
        SET 
            event_id = COALESCE(?, event_id),
            question_text = COALESCE(?, question_text),
            explanation = COALESCE(?, explanation),
            difficulty_level = COALESCE(?, difficulty_level)
        WHERE id = ?
        RETURNING *
        "#
    )
    .bind(payload.event_id)
    .bind(payload.question_text)
    .bind(payload.explanation)
    .bind(payload.difficulty_level)
    .bind(id)
    .fetch_optional(&pool)
    .await;

    match result {
        Ok(Some(q)) => (StatusCode::OK, Json(q)).into_response(),
        Ok(None) => (StatusCode::NOT_FOUND, "Question not found").into_response(),
        Err(e) => {
            tracing::error!("Failed to update question: {:?}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, "Failed to update question").into_response()
        }
    }
}

#[utoipa::path(
    delete,
    path = "/questions/{id}",
    params(
        ("id" = i64, Path, description = "Question ID"),
        ("x-api-key" = String, Header, description = "API Key for authentication")
    ),
    responses(
        (status = 204, description = "Question deleted successfully"),
        (status = 401, description = "Unauthorized"),
        (status = 404, description = "Question not found"),
        (status = 500, description = "Internal Server Error")
    )
)]
pub async fn delete_question(
    State(pool): State<DbPool>,
    Path(id): Path<i64>,
    _api_key: ApiKey,
) -> impl IntoResponse {
    // Delete options first (Manual Cascade)
    let _ = sqlx::query("DELETE FROM options WHERE question_id = ?")
        .bind(id)
        .execute(&pool)
        .await; // Ignore error? Or fail? Best to log.

    // Then delete question
    let result = sqlx::query("DELETE FROM questions WHERE id = ?")
        .bind(id)
        .execute(&pool)
        .await;

    match result {
        Ok(res) => {
            if res.rows_affected() > 0 {
                (StatusCode::NO_CONTENT, ()).into_response()
            } else {
                (StatusCode::NOT_FOUND, "Question not found").into_response()
            }
        }
        Err(e) => {
            tracing::error!("Failed to delete question: {:?}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, "Failed to delete question").into_response()
        }
    }
}
