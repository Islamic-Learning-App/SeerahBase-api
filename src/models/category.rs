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
pub struct CreateCategory {
    pub name: String,
    pub name_bn: String,
    pub category_type: String,
    pub description: Option<String>,
    pub description_bn: Option<String>,
    pub icon: Option<String>,
    pub sort_order: i64,
    pub parent_id: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct UpdateCategory {
    pub name: Option<String>,
    pub name_bn: Option<String>,
    pub category_type: Option<String>,
    pub description: Option<String>,
    pub description_bn: Option<String>,
    pub icon: Option<String>,
    pub sort_order: Option<i64>,
    pub parent_id: Option<i64>,
}
