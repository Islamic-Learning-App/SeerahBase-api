use crate::{
    auth::ApiKey,
    db::DbPool,
    errors::AppError,
    models::{
        Category, CreateEvent, Event, PaginatedResponse, PaginationParams, UpdateEvent,
    },
};
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};

#[utoipa::path(
    get,
    path = "/categories",
    params(
        ("type" = Option<String>, Query, description = "Filter by category type (era, prophet, etc)")
    ),
    responses(
        (status = 200, description = "List categories", body = Vec<Category>)
    )
)]
pub async fn get_categories(
    State(db): State<DbPool>,
    Query(params): Query<std::collections::HashMap<String, String>>,
) -> Result<Json<Vec<Category>>, AppError> {
    let conn = db.connect()?;
    
    let mut query = "SELECT * FROM categories".to_string();
    let mut args = Vec::new();
    
    if let Some(cat_type) = params.get("type") {
        query.push_str(" WHERE category_type = ?");
        args.push(cat_type.as_str());
    }
    
    query.push_str(" ORDER BY sort_order ASC");

    // libsql execute/query args handling is a bit specific. 
    // using params! macro or a slice of values.
    // For dynamic args, we might need a different approach or just execute raw if safe/simple, 
    // but better to use parameterized.
    // libsql::params::Params::from(args) ?

    let mut rows = if args.is_empty() {
        conn.query(&query, ()).await?
    } else {
        conn.query(&query, libsql::params![args[0].to_string()]).await?
    };

    let mut categories = Vec::new();
    while let Some(row) = rows.next().await? {
        categories.push(Category::from_row(&row)?);
    }

    Ok(Json(categories))
}

#[utoipa::path(
    get,
    path = "/categories/{id}/events",
    params(
        ("id" = i64, Path, description = "Category ID")
    ),
    responses(
        (status = 200, description = "List events by category", body = Vec<Event>)
    )
)]
pub async fn get_events_by_category(
    State(db): State<DbPool>,
    Path(id): Path<i64>,
) -> Result<Json<Vec<Event>>, AppError> {
    let conn = db.connect()?;
    
    let mut rows = conn
        .query(
            "SELECT * FROM events WHERE category_id = ? ORDER BY event_date ASC",
            libsql::params![id],
        )
        .await?;

    let mut events = Vec::new();
    while let Some(row) = rows.next().await? {
        events.push(Event::from_row(&row)?);
    }

    Ok(Json(events))
}

#[utoipa::path(
    get,
    path = "/events",
    params(
        PaginationParams
    ),
    responses(
        (status = 200, description = "List all events paginated", body = PaginatedResponse<Event>)
    )
)]
pub async fn get_all_events(
    State(db): State<DbPool>,
    Query(pagination): Query<PaginationParams>,
) -> Result<Json<PaginatedResponse<Event>>, AppError> {
    let conn = db.connect()?;
    
    let page = pagination.page.unwrap_or(1).max(1);
    let limit = pagination.limit.unwrap_or(20).clamp(1, 100);
    let offset = (page - 1) * limit;

    // Get total count
    let mut count_rows = conn.query("SELECT COUNT(*) FROM events", ()).await?;
    let total: i64 = if let Some(row) = count_rows.next().await? {
        row.get(0)?
    } else {
        0
    };

    // Get data
    let mut rows = conn
        .query(
            "SELECT * FROM events ORDER BY event_date ASC LIMIT ? OFFSET ?",
            libsql::params![limit as i64, offset as i64],
        )
        .await?;

    let mut events = Vec::new();
    while let Some(row) = rows.next().await? {
        events.push(Event::from_row(&row)?);
    }

    Ok(Json(PaginatedResponse {
        data: events,
        page,
        limit,
        total: total as u64,
    }))
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
    State(db): State<DbPool>,
    _api_key: ApiKey,
    Json(payload): Json<CreateEvent>,
) -> Result<impl IntoResponse, AppError> {
    let conn = db.connect()?;

    let mut rows = conn
        .query(
            "INSERT INTO events (category_id, title, title_bn, description, description_bn, event_date, source, image_url) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8) RETURNING *",
            libsql::params![
                payload.category_id,
                payload.title,
                payload.title_bn,
                payload.description,
                payload.description_bn,
                payload.event_date,
                payload.source,
                payload.image_url
            ],
        )
        .await?;

    if let Some(row) = rows.next().await? {
        let event = Event::from_row(&row)?;
        Ok((StatusCode::CREATED, Json(event)))
    } else {
        Err(AppError::InternalServerError("Failed to return created event".to_string()))
    }
}

// TODO: Update and Delete similarly... 
// For brevity in this turn, I implemented Create. Update/Delete follow same pattern.
// I'll add them to be thorough.

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
        (status = 404, description = "Event not found")
    )
)]
pub async fn update_event(
    State(db): State<DbPool>,
    Path(id): Path<i64>,
    _api_key: ApiKey,
    Json(payload): Json<UpdateEvent>,
) -> Result<impl IntoResponse, AppError> {
    let conn = db.connect()?;

    // Using COALESCE logic in SQL or building the query.
    // Libsql might not support named params easily or COALESCE with bindings for partial updates nicely without boilerplate.
    // Let's use simple COALESCE query.

    let mut rows = conn
        .query(
            r#"
            UPDATE events 
            SET 
                category_id = COALESCE(?1, category_id),
                title = COALESCE(?2, title),
                title_bn = COALESCE(?3, title_bn),
                description = COALESCE(?4, description),
                description_bn = COALESCE(?5, description_bn),
                event_date = COALESCE(?6, event_date),
                source = COALESCE(?7, source),
                image_url = COALESCE(?8, image_url)
            WHERE id = ?9
            RETURNING *
            "#,
            libsql::params![
                payload.category_id,
                payload.title,
                payload.title_bn,
                payload.description,
                payload.description_bn,
                payload.event_date,
                payload.source,
                payload.image_url,
                id
            ],
        )
        .await?;

    if let Some(row) = rows.next().await? {
        let event = Event::from_row(&row)?;
        Ok(Json(event))
    } else {
        Err(AppError::NotFound("Event not found".to_string()))
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
        (status = 404, description = "Event not found")
    )
)]
pub async fn delete_event(
    State(db): State<DbPool>,
    Path(id): Path<i64>,
    _api_key: ApiKey,
) -> Result<impl IntoResponse, AppError> {
    let conn = db.connect()?;

    // Manual cascading?
    conn.execute("DELETE FROM options WHERE question_id IN (SELECT id FROM questions WHERE event_id = ?1)", libsql::params![id]).await?;
    conn.execute("DELETE FROM questions WHERE event_id = ?1", libsql::params![id]).await?;

    let result = conn.execute("DELETE FROM events WHERE id = ?1", libsql::params![id]).await?;

    if result > 0 {
        Ok((StatusCode::NO_CONTENT, ()))
    } else {
        Err(AppError::NotFound("Event not found".to_string()))
    }
}

#[utoipa::path(
    get,
    path = "/events/{id}",
    params(
        ("id" = i64, Path, description = "Event ID")
    ),
    responses(
        (status = 200, description = "Event details", body = Event),
        (status = 404, description = "Event not found")
    )
)]
pub async fn get_event_by_id(
    State(db): State<DbPool>,
    Path(id): Path<i64>,
) -> Result<impl IntoResponse, AppError> {
    let conn = db.connect()?;

    let mut rows = conn
        .query("SELECT * FROM events WHERE id = ?", libsql::params![id])
        .await?;

    if let Some(row) = rows.next().await? {
        let event = Event::from_row(&row)?;
        Ok(Json(event))
    } else {
        Err(AppError::NotFound("Event not found".to_string()))
    }
}
