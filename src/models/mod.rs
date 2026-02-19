use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct Category {
    pub id: i64,
    pub name: String,
    pub name_bn: String,
    pub category_type: String,
    pub description: Option<String>,
    pub description_bn: Option<String>,
    pub icon: Option<String>,
    pub sort_order: i64,
    pub parent_id: Option<i64>,
}

impl Category {
    pub fn from_row(row: &libsql::Row) -> Result<Self, libsql::Error> {
        Ok(Self {
            id: row.get(0)?,
            name: row.get(1)?,
            name_bn: row.get(2)?,
            category_type: row.get(3)?,
            description: row.get(4).unwrap_or(None),
            description_bn: row.get(5).unwrap_or(None),
            icon: row.get(6).unwrap_or(None),
            sort_order: row.get(7).unwrap_or(0),
            parent_id: row.get(8).unwrap_or(None),
        })
    }
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct Event {
    pub id: i64,
    pub category_id: Option<i64>,
    pub title: String,
    pub title_bn: Option<String>,
    pub description: String,
    pub description_bn: Option<String>,
    pub event_date: Option<String>,
    pub source: Option<String>,
    pub image_url: Option<String>,
}

impl Event {
    pub fn from_row(row: &libsql::Row) -> Result<Self, libsql::Error> {
        Ok(Self {
            id: row.get(0)?,
            category_id: row.get(1).unwrap_or(None),
            title: row.get(2)?,
            title_bn: row.get(3).unwrap_or(None),
            description: row.get(4)?,
            description_bn: row.get(5).unwrap_or(None),
            event_date: row.get(6).unwrap_or(None),
            source: row.get(7).unwrap_or(None),
            image_url: row.get(8).unwrap_or(None),
        })
    }
}

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

// Helper structs for creation/updates (simplified for now)
#[derive(Debug, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CreateEvent {
    pub category_id: Option<i64>,
    pub title: String,
    pub title_bn: Option<String>,
    pub description: String,
    pub description_bn: Option<String>,
    pub event_date: Option<String>,
    pub source: Option<String>,
    pub image_url: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct UpdateEvent {
    pub category_id: Option<i64>,
    pub title: Option<String>,
    pub title_bn: Option<String>,
    pub description: Option<String>,
    pub description_bn: Option<String>,
    pub event_date: Option<String>,
    pub source: Option<String>,
    pub image_url: Option<String>,
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

// Pagination
#[derive(Debug, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct PaginatedResponse<T> {
    pub data: Vec<T>,
    pub page: u64,
    pub limit: u64,
    pub total: u64,
}

use utoipa::IntoParams;

#[derive(Debug, Deserialize, ToSchema, IntoParams)]
pub struct PaginationParams {
    pub page: Option<u64>,
    pub limit: Option<u64>,
}
