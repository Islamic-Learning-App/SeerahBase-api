use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

#[derive(Debug, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct PaginatedResponse<T> {
    pub data: Vec<T>,
    pub page: u64,
    pub limit: u64,
    pub total: u64,
}

#[derive(Debug, Deserialize, ToSchema, IntoParams)]
pub struct PaginationParams {
    pub page: Option<u64>,
    pub limit: Option<u64>,
}
