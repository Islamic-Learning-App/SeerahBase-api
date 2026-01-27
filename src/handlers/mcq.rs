use axum::{
    extract::{Path, State},
    Json,
};
use crate::{db::DbPool, models::{Question, AnswerOption, QuestionWithOptions}};

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
        let options = sqlx::query_as::<_, AnswerOption>("SELECT * FROM options WHERE question_id = ?")
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
    let questions = sqlx::query_as::<_, Question>("SELECT * FROM questions ORDER BY RANDOM() LIMIT 5")
        .fetch_all(&pool)
        .await
        .unwrap_or_else(|_| vec![]);

    let mut result = Vec::new();
    for q in questions {
        let options = sqlx::query_as::<_, AnswerOption>("SELECT * FROM options WHERE question_id = ?")
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
