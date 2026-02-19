use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct Question {
    pub id: i64,
    pub event_id: Option<i64>,
    pub category_id: Option<i64>,
    pub question_text: String,
    pub question_text_bn: Option<String>,
    pub explanation: Option<String>,
    pub explanation_bn: Option<String>,
    pub difficulty_level: Option<String>,
}

impl Question {
    pub fn from_row(row: &libsql::Row) -> Result<Self, libsql::Error> {
        Ok(Self {
            id: row.get(0)?,
            event_id: row.get(1).unwrap_or(None),
            category_id: row.get(2).unwrap_or(None),
            question_text: row.get(3)?,
            question_text_bn: row.get(4).unwrap_or(None),
            explanation: row.get(5).unwrap_or(None),
            explanation_bn: row.get(6).unwrap_or(None),
            difficulty_level: row.get(7).unwrap_or(None),
        })
    }
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct AnswerOption {
    pub id: i64,
    pub question_id: i64,
    pub option_text: String,
    pub option_text_bn: Option<String>,
    pub is_correct: bool,
}

impl AnswerOption {
    pub fn from_row(row: &libsql::Row) -> Result<Self, libsql::Error> {
        Ok(Self {
            id: row.get(0)?,
            question_id: row.get(1)?,
            option_text: row.get(2)?,
            option_text_bn: row.get(3).unwrap_or(None),
            is_correct: row.get(4)?,
        })
    }
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct QuestionWithOptions {
    #[serde(flatten)]
    pub question: Question,
    pub options: Vec<AnswerOption>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CreateQuestion {
    pub event_id: Option<i64>,
    pub category_id: Option<i64>,
    pub question_text: String,
    pub question_text_bn: Option<String>,
    pub explanation: Option<String>,
    pub explanation_bn: Option<String>,
    pub difficulty_level: Option<String>,
    pub options: Vec<CreateOption>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CreateOption {
    pub option_text: String,
    pub option_text_bn: Option<String>,
    pub is_correct: bool,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct UpdateQuestion {
    pub event_id: Option<i64>,
    pub category_id: Option<i64>,
    pub question_text: Option<String>,
    pub question_text_bn: Option<String>,
    pub explanation: Option<String>,
    pub explanation_bn: Option<String>,
    pub difficulty_level: Option<String>,
}
