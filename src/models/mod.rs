use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, FromRow, ToSchema)]
pub struct Era {
    pub id: i64,
    pub name: String,
    pub description: Option<String>,
    pub start_date: Option<String>,
    pub end_date: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, FromRow, ToSchema)]
pub struct Event {
    pub id: i64,
    pub era_id: Option<i64>,
    pub title: String,
    pub description: String,
    pub event_date: Option<String>,
    pub source: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, FromRow, ToSchema)]
pub struct Question {
    pub id: i64,
    pub event_id: Option<i64>,
    pub question_text: String,
    pub explanation: Option<String>,
    pub difficulty_level: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, FromRow, ToSchema)]
pub struct AnswerOption {
    pub id: i64,
    pub question_id: i64,
    pub option_text: String,
    pub is_correct: bool,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct QuestionWithOptions {
    #[serde(flatten)]
    pub question: Question,
    pub options: Vec<AnswerOption>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct CreateEvent {
    pub era_id: Option<i64>,
    pub title: String,
    pub description: String,
    pub event_date: Option<String>,
    pub source: Option<String>,
}
