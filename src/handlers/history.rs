use crate::{
    auth::ApiKey,
    db::DbPool,
    models::{CreateEvent, UpdateEvent, Era, Event},
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

#[utoipa::path(
    put,
    path = "/events/{id}",
    request_body = UpdateEvent,
    params(
        ("id" = i64, Path, description = "Event ID"),
        ("x-api-key" = String, Header, description = "API Key for authentication")
    ),
    responses(
        (status = 200, description = "Event updated successfully", body = Event),
        (status = 401, description = "Unauthorized"),
        (status = 404, description = "Event not found"),
        (status = 500, description = "Internal Server Error")
    )
)]
pub async fn update_event(
    State(pool): State<DbPool>,
    Path(id): Path<i64>,
    _api_key: ApiKey,
    Json(payload): Json<UpdateEvent>,
) -> impl IntoResponse {
    /* 
       Previously planned dynamic query builder logic removed in favor of COALESCE.
       Unused variables cleaned up.
    */
    
    let result = sqlx::query_as::<_, Event>(
        r#"
        UPDATE events 
        SET 
            era_id = COALESCE(?, era_id),
            title = COALESCE(?, title),
            description = COALESCE(?, description),
            event_date = COALESCE(?, event_date),
            source = COALESCE(?, source)
        WHERE id = ?
        RETURNING *
        "#
    )
    .bind(payload.era_id)
    .bind(payload.title)
    .bind(payload.description)
    .bind(payload.event_date)
    .bind(payload.source)
    .bind(id)
    .fetch_optional(&pool)
    .await;

    match result {
        Ok(Some(event)) => (StatusCode::OK, Json(event)).into_response(),
        Ok(None) => (StatusCode::NOT_FOUND, "Event not found").into_response(),
        Err(e) => {
            tracing::error!("Failed to update event: {:?}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, "Failed to update event").into_response()
        }
    }
}

#[utoipa::path(
    delete,
    path = "/events/{id}",
    params(
        ("id" = i64, Path, description = "Event ID"),
        ("x-api-key" = String, Header, description = "API Key for authentication")
    ),
    responses(
        (status = 204, description = "Event deleted successfully"),
        (status = 401, description = "Unauthorized"),
        (status = 404, description = "Event not found"), // SQLx execution result usually returns rows affected
        (status = 500, description = "Internal Server Error")
    )
)]
pub async fn delete_event(
    State(pool): State<DbPool>,
    Path(id): Path<i64>,
    _api_key: ApiKey,
) -> impl IntoResponse {
    let mut tx = match pool.begin().await {
        Ok(tx) => tx,
        Err(e) => {
            tracing::error!("Failed to start transaction: {:?}", e);
            return (StatusCode::INTERNAL_SERVER_ERROR, "Failed to start transaction").into_response();
        }
    };

    // 1. Delete Options for Questions related to this event
    if let Err(e) = sqlx::query("DELETE FROM options WHERE question_id IN (SELECT id FROM questions WHERE event_id = ?)")
        .bind(id)
        .execute(&mut *tx)
        .await
    {
        tracing::error!("Failed to delete options: {:?}", e);
        return (StatusCode::INTERNAL_SERVER_ERROR, "Failed to delete dependent options").into_response();
    }

    // 2. Delete Questions related to this event
    if let Err(e) = sqlx::query("DELETE FROM questions WHERE event_id = ?")
        .bind(id)
        .execute(&mut *tx)
        .await
    {
        tracing::error!("Failed to delete questions: {:?}", e);
        return (StatusCode::INTERNAL_SERVER_ERROR, "Failed to delete dependent questions").into_response();
    }

    // 3. Delete the Event itself
    let result = sqlx::query("DELETE FROM events WHERE id = ?")
        .bind(id)
        .execute(&mut *tx)
        .await;

    match result {
        Ok(res) => {
            if res.rows_affected() > 0 {
                if let Err(e) = tx.commit().await {
                     tracing::error!("Failed to commit transaction: {:?}", e);
                     return (StatusCode::INTERNAL_SERVER_ERROR, "Failed to commit transaction").into_response();
                }
                (StatusCode::NO_CONTENT, ()).into_response()
            } else {
                (StatusCode::NOT_FOUND, "Event not found").into_response()
            }
        }
        Err(e) => {
            tracing::error!("Failed to delete event: {:?}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, "Failed to delete event").into_response()
        }
    }
}
