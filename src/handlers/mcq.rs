use crate::{
    auth::ApiKey,
    db::{DbPool, sync_db},
    errors::AppError,
    models::{
        AnswerOption, CreateQuestion, Question, QuestionWithOptions,
    },
};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use std::collections::HashMap;

#[utoipa::path(
    get,
    path = "/events/{id}/quiz",
    params(
        ("id" = i64, Path, description = "Event ID")
    ),
    responses(
        (status = 200, description = "List questions for an event", body = Vec<QuestionWithOptions>)
    )
)]
pub async fn get_questions_by_event(
    State(db): State<DbPool>,
    Path(event_id): Path<i64>,
) -> Result<Json<Vec<QuestionWithOptions>>, AppError> {
    let conn = db.connect()?;

    // Fix N+1: Fetch questions and options in one go or two efficient queries.
    // Approach: Fetch questions, then fetch all options for these questions.
    
    // 1. Fetch Questions
    let mut q_rows = conn
        .query("SELECT * FROM questions WHERE event_id = ?", libsql::params![event_id])
        .await?;
    
    let mut questions = Vec::new();
    let mut question_ids = Vec::new();
    
    while let Some(row) = q_rows.next().await? {
        let q = Question::from_row(&row)?;
        question_ids.push(q.id);
        questions.push(q);
    }
    
    if questions.is_empty() {
        return Ok(Json(vec![]));
    }

    // 2. Fetch Options for these questions
    // "WHERE question_id IN (...)" dynamically built
    let placeholders = question_ids.iter().map(|_| "?").collect::<Vec<_>>().join(",");
    let query = format!("SELECT * FROM options WHERE question_id IN ({})", placeholders);
    
    // Combine params
    let params: Vec<libsql::Value> = question_ids.iter().map(|&id| id.into()).collect();
    
    let mut o_rows = conn.query(&query, libsql::params::Params::Positional(params)).await?;
    
    let mut options_map: HashMap<i64, Vec<AnswerOption>> = HashMap::new();
    while let Some(row) = o_rows.next().await? {
        let opt = AnswerOption::from_row(&row)?;
        options_map.entry(opt.question_id).or_default().push(opt);
    }

    // 3. Assemble
    let result = questions.into_iter().map(|q| {
        let opts = options_map.remove(&q.id).unwrap_or_default();
        QuestionWithOptions {
            question: q,
            options: opts,
        }
    }).collect();

    Ok(Json(result))
}

#[utoipa::path(
    get,
    path = "/quiz/random",
    responses(
        (status = 200, description = "Get random quiz questions", body = Vec<QuestionWithOptions>)
    )
)]
pub async fn get_random_quiz(
    State(db): State<DbPool>,
) -> Result<Json<Vec<QuestionWithOptions>>, AppError> {
    let conn = db.connect()?;

    // Fetch 10 random questions
    let mut q_rows = conn
        .query("SELECT * FROM questions ORDER BY RANDOM() LIMIT 10", ())
        .await?;
    
    let mut questions = Vec::new();
    let mut question_ids = Vec::new();
    
    while let Some(row) = q_rows.next().await? {
        let q = Question::from_row(&row)?;
        question_ids.push(q.id);
        questions.push(q);
    }
    
    if questions.is_empty() {
        return Ok(Json(vec![]));
    }

    // Fetch options
    let placeholders = question_ids.iter().map(|_| "?").collect::<Vec<_>>().join(",");
    let query = format!("SELECT * FROM options WHERE question_id IN ({})", placeholders);
    
    let params: Vec<libsql::Value> = question_ids.iter().map(|&id| id.into()).collect();
    let mut o_rows = conn.query(&query, libsql::params::Params::Positional(params)).await?;
    
    let mut options_map: HashMap<i64, Vec<AnswerOption>> = HashMap::new();
    while let Some(row) = o_rows.next().await? {
        let opt = AnswerOption::from_row(&row)?;
        options_map.entry(opt.question_id).or_default().push(opt);
    }

    let result = questions.into_iter().map(|q| {
        let opts = options_map.remove(&q.id).unwrap_or_default();
        QuestionWithOptions {
            question: q,
            options: opts,
        }
    }).collect();

    Ok(Json(result))
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
    State(db): State<DbPool>,
    _api_key: ApiKey,
    Json(payload): Json<CreateQuestion>,
) -> Result<impl IntoResponse, AppError> {
    let conn = db.connect()?;

    // Transaction?
    // libsql remote doesn't support interactive transactions easily unless using a specific client mode or `batch`.
    // We'll do sequential checks/inserts. If one fails, we might leave orphan partial data, but 
    // for this app, simplicity is preferred over strict ACID for now with remote HTTP.
    // Or we can try executing a batched SQL string if helper supports it, but standard query() doesn't.
    // Let's stick to sequential.

    let mut q_rows = conn
        .query(
            "INSERT INTO questions (event_id, category_id, question_text, question_text_bn, explanation, explanation_bn, difficulty_level) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7) RETURNING *",
            libsql::params![
                payload.event_id,
                payload.category_id,
                payload.question_text,
                payload.question_text_bn,
                payload.explanation,
                payload.explanation_bn,
                payload.difficulty_level
            ],
        )
        .await?;

    let question = if let Some(row) = q_rows.next().await? {
        Question::from_row(&row)?
    } else {
        return Err(AppError::InternalServerError("Failed to create question".to_string()));
    };

    let mut created_options = Vec::new();
    for opt in payload.options {
        let mut o_rows = conn
            .query(
                "INSERT INTO options (question_id, option_text, option_text_bn, is_correct) VALUES (?1, ?2, ?3, ?4) RETURNING *",
                libsql::params![question.id, opt.option_text, opt.option_text_bn, opt.is_correct],
            )
            .await?;
        
        if let Some(row) = o_rows.next().await? {
            created_options.push(AnswerOption::from_row(&row)?);
        }
    }

    sync_db(&db).await;

    Ok((StatusCode::CREATED, Json(QuestionWithOptions {
        question,
        options: created_options,
    })))
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
        (status = 404, description = "Question not found")
    )
)]
pub async fn delete_question(
    State(db): State<DbPool>,
    Path(id): Path<i64>,
    _api_key: ApiKey,
) -> Result<impl IntoResponse, AppError> {
    let conn = db.connect()?;

    conn.execute("DELETE FROM options WHERE question_id = ?1", libsql::params![id]).await?;
    let result = conn.execute("DELETE FROM questions WHERE id = ?1", libsql::params![id]).await?;

    if result > 0 {
        sync_db(&db).await;
        Ok((StatusCode::NO_CONTENT, ()))
    } else {
        Err(AppError::NotFound("Question not found".to_string()))
    }
}
