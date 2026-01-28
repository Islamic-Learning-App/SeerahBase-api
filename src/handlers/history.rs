use crate::{
    auth::ApiKey,
    db::DbPool,
    models::{CreateEvent, Era, Event},
};
use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};

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
    let events =
        sqlx::query_as::<_, Event>("SELECT * FROM events WHERE era_id = ? ORDER BY event_date")
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

#[utoipa::path(
    post,
    path = "/events",
    request_body = CreateEvent,
    params(
        ("x-api-key" = String, Header, description = "API Key for authentication")
    ),
    responses(
        (status = 201, description = "Event created successfully", body = Event),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal Server Error")
    )
)]
pub async fn create_event(
    State(pool): State<DbPool>,
    _api_key: ApiKey,
    Json(payload): Json<CreateEvent>,
) -> impl IntoResponse {
    let result = sqlx::query_as::<_, Event>(
        "INSERT INTO events (era_id, title, description, event_date, source) VALUES (?, ?, ?, ?, ?) RETURNING *"
    )
    .bind(payload.era_id)
    .bind(payload.title)
    .bind(payload.description)
    .bind(payload.event_date)
    .bind(payload.source)
    .fetch_one(&pool)
    .await;

    match result {
        Ok(event) => (StatusCode::CREATED, Json(event)).into_response(),
        Err(e) => {
            tracing::error!("Failed to create event: {:?}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, "Failed to create event").into_response()
        }
    }
}
