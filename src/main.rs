mod auth;
mod db;
mod errors;
mod handlers;
mod models;

use axum::{
    routing::{delete, get, post},
    Router,
};
use std::net::SocketAddr;
use tower_http::{
    compression::CompressionLayer,
    cors::{Any, CorsLayer},
    timeout::TimeoutLayer,
};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::models::*;

#[derive(OpenApi)]
#[openapi(
    paths(
        handlers::history::get_categories,
        handlers::history::create_category,
        handlers::history::update_category,
        handlers::history::delete_category,
        handlers::history::get_events_by_category,
        handlers::history::get_all_events,
        handlers::history::create_event,
        handlers::history::update_event,
        handlers::history::delete_event,
        handlers::history::get_event_by_id,
        handlers::mcq::get_questions_by_event,
        handlers::mcq::get_random_quiz,
        handlers::mcq::create_question,
        handlers::mcq::delete_question,
    ),
    components(
        schemas(
            Category, CreateCategory, UpdateCategory,
            Event, CreateEvent, UpdateEvent, 
            Question, QuestionWithOptions, AnswerOption, CreateQuestion, CreateOption,
            PaginatedResponse<Event>, PaginationParams
        )
    ),
    tags(
        (name = "SeerahBase", description = "Islamic History API with Categories")
    )
)]
struct ApiDoc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "seerah_base=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Initialize Libsql DB (Remote)
    let db = db::init_db().await?;
    
    // Ensure API_KEY is set
    if std::env::var("API_KEY").is_err() {
        tracing::warn!("API_KEY not set in environment");
    }

    let app = Router::new()
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .route("/categories", get(handlers::history::get_categories).post(handlers::history::create_category))
        .route("/categories/{id}", axum::routing::put(handlers::history::update_category).delete(handlers::history::delete_category))
        .route("/categories/{id}/events", get(handlers::history::get_events_by_category))
        .route("/events", get(handlers::history::get_all_events).post(handlers::history::create_event))
        .route("/events/{id}", get(handlers::history::get_event_by_id).put(handlers::history::update_event).delete(handlers::history::delete_event))
        .route("/events/{id}/quiz", get(handlers::mcq::get_questions_by_event))
        .route("/quiz/random", get(handlers::mcq::get_random_quiz))
        .route("/questions", post(handlers::mcq::create_question))
        .route("/questions/{id}", delete(handlers::mcq::delete_question))
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods(Any)
                .allow_headers(Any),
        )
        .layer(CompressionLayer::new())
        .layer(TimeoutLayer::new(std::time::Duration::from_secs(10)))
        .with_state(db);

    let port = std::env::var("PORT").unwrap_or_else(|_| "3000".to_string());
    let addr: SocketAddr = format!("0.0.0.0:{}", port).parse()?;

    tracing::info!("listening on {}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
