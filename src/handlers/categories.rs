use crate::{
    auth::ApiKey,
    db::{DbPool, sync_db},
    errors::AppError,
    models::{Category, CreateCategory, UpdateCategory},
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
    post,
    path = "/categories",
    request_body = CreateCategory,
    params(
        ("x-api-key" = String, Header, description = "API Key for authentication")
    ),
    responses(
        (status = 201, description = "Category created successfully", body = Category),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal Server Error")
    )
)]
pub async fn create_category(
    State(db): State<DbPool>,
    _api_key: ApiKey,
    Json(payload): Json<CreateCategory>,
) -> Result<impl IntoResponse, AppError> {
    let conn = db.connect()?;

    let mut rows: libsql::Rows = conn
        .query(
            "INSERT INTO categories (name, name_bn, category_type, description, description_bn, icon, sort_order, parent_id) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8) RETURNING *",
            libsql::params![
                payload.name,
                payload.name_bn,
                payload.category_type,
                payload.description,
                payload.description_bn,
                payload.icon,
                payload.sort_order,
                payload.parent_id
            ],
        )
        .await?;

    if let Some(row) = rows.next().await? {
        let category = Category::from_row(&row)?;
        sync_db(&db).await;
        Ok((StatusCode::CREATED, Json(category)))
    } else {
        Err(AppError::InternalServerError("Failed to return created category".to_string()))
    }
}

#[utoipa::path(
    put,
    path = "/categories/{id}",
    request_body = UpdateCategory,
    params(
        ("id" = i64, Path, description = "Category ID"),
        ("x-api-key" = String, Header, description = "API Key for authentication")
    ),
    responses(
        (status = 200, description = "Category updated successfully", body = Category),
        (status = 401, description = "Unauthorized"),
        (status = 404, description = "Category not found")
    )
)]
pub async fn update_category(
    State(db): State<DbPool>,
    Path(id): Path<i64>,
    _api_key: ApiKey,
    Json(payload): Json<UpdateCategory>,
) -> Result<impl IntoResponse, AppError> {
    let conn = db.connect()?;

    let mut rows = conn
        .query(
            r#"
            UPDATE categories 
            SET 
                name = COALESCE(?1, name),
                name_bn = COALESCE(?2, name_bn),
                category_type = COALESCE(?3, category_type),
                description = COALESCE(?4, description),
                description_bn = COALESCE(?5, description_bn),
                icon = COALESCE(?6, icon),
                sort_order = COALESCE(?7, sort_order),
                parent_id = COALESCE(?8, parent_id)
            WHERE id = ?9
            RETURNING *
            "#,
            libsql::params![
                payload.name,
                payload.name_bn,
                payload.category_type,
                payload.description,
                payload.description_bn,
                payload.icon,
                payload.sort_order,
                payload.parent_id,
                id
            ],
        )
        .await?;

    if let Some(row) = rows.next().await? {
        let category = Category::from_row(&row)?;
        sync_db(&db).await;
        Ok(Json(category))
    } else {
        Err(AppError::NotFound("Category not found".to_string()))
    }
}

#[utoipa::path(
    delete,
    path = "/categories/{id}",
    params(
        ("id" = i64, Path, description = "Category ID"),
        ("x-api-key" = String, Header, description = "API Key for authentication")
    ),
    responses(
        (status = 204, description = "Category deleted successfully"),
        (status = 401, description = "Unauthorized"),
        (status = 404, description = "Category not found")
    )
)]
pub async fn delete_category(
    State(db): State<DbPool>,
    Path(id): Path<i64>,
    _api_key: ApiKey,
) -> Result<impl IntoResponse, AppError> {
    let conn = db.connect()?;

    let mut check_events = conn
        .query("SELECT COUNT(*) FROM events WHERE category_id = ?", libsql::params![id])
        .await?;
    if let Some(row) = check_events.next().await? {
        let count: i64 = row.get(0)?;
        if count > 0 {
            return Err(AppError::InternalServerError(
                "Cannot delete category with existing events".to_string(),
            ));
        }
    }

    let result = conn
        .execute("DELETE FROM categories WHERE id = ?1", libsql::params![id])
        .await?;

    if result > 0 {
        sync_db(&db).await;
        Ok((StatusCode::NO_CONTENT, ()))
    } else {
        Err(AppError::NotFound("Category not found".to_string()))
    }
}
