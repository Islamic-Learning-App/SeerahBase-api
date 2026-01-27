use axum::{
    routing::get,
    Router,
};
use std::net::SocketAddr;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;
use tower_http::compression::CompressionLayer;

mod db;
mod handlers;
mod models;

#[derive(OpenApi)]
#[openapi(
    paths(
        handlers::history::get_eras,
        handlers::history::get_events_by_era,
        handlers::history::get_all_events,
        handlers::mcq::get_questions_by_event,
        handlers::mcq::get_random_quiz,
    ),
    components(
        schemas(models::Era, models::Event, models::Question, models::AnswerOption, models::QuestionWithOptions)
    ),
    tags(
        (name = "SeerahBase", description = "SeerahBase API Endpoints")
    )
)]
struct ApiDoc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();

    // Initialize tracing
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .with(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    // Initialize DB connection
    let pool = db::init_pool().await?;
    
    // Build our application with routes
    let app = Router::new()
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .route("/", get(root))
        .route("/eras", get(handlers::history::get_eras))
        .route("/eras/{id}/events", get(handlers::history::get_events_by_era))
        .route("/events", get(handlers::history::get_all_events))
        .route("/questions/event/{id}", get(handlers::mcq::get_questions_by_event))
        .route("/questions/random", get(handlers::mcq::get_random_quiz))
        .layer(CompressionLayer::new())
        .with_state(pool);

    // Run it
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::info!("listening on {}", addr);
    
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    axum::serve(listener, app).await?;
    
    Ok(())
}

async fn root() -> &'static str {
    "SeerahBase API is running. Endpoints: /eras, /events, /questions/random"
}
