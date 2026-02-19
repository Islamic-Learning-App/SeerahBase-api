use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

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
