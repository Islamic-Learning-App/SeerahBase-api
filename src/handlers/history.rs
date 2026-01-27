use axum::{
    extract::{Path, State},
    Json,
};
use crate::{db::DbPool, models::{Era, Event}};

#[utoipa::path(
    get,
    path = "/eras",
    responses(
        (status = 200, description = "List all eras", body = Vec<Era>)
    )
)]
pub async fn get_eras(State(pool): State<DbPool>) -> Json<Vec<Era>> {
    let eras = sqlx::query_as::<_, Era>("SELECT * FROM eras ORDER BY start_date")
        .fetch_all(&pool)
        .await
        .unwrap_or_else(|_| vec![]);

    Json(eras)
}

#[utoipa::path(
    get,
    path = "/eras/{id}/events",
    params(
        ("id" = i64, Path, description = "Era ID")
    ),
    responses(
        (status = 200, description = "List events by era", body = Vec<Event>)
    )
)]
pub async fn get_events_by_era(
    State(pool): State<DbPool>,
    Path(era_id): Path<i64>,
) -> Json<Vec<Event>> {
    let events = sqlx::query_as::<_, Event>("SELECT * FROM events WHERE era_id = ? ORDER BY event_date")
        .bind(era_id)
        .fetch_all(&pool)
        .await
        .unwrap_or_else(|_| vec![]);

    Json(events)
}

#[utoipa::path(
    get,
    path = "/events",
    responses(
        (status = 200, description = "List all events", body = Vec<Event>)
    )
)]
pub async fn get_all_events(State(pool): State<DbPool>) -> Json<Vec<Event>> {
    let events = sqlx::query_as::<_, Event>("SELECT * FROM events ORDER BY event_date")
        .fetch_all(&pool)
        .await
        .unwrap_or_else(|_| vec![]);

    Json(events)
}
