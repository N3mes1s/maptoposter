use std::net::SocketAddr;
use std::sync::Arc;

use axum::{
    routing::{get, post},
    Router,
};
use tower_http::cors::{Any, CorsLayer};
use tower_http::services::ServeDir;
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod api;
mod config;
mod core;
mod error;
mod rendering;
mod themes;

use api::state::AppState;

#[tokio::main]
async fn main() {
    // Initialize tracing
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "maptoposter_rs=info,tower_http=info".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Load configuration
    let config = config::Settings::from_env();
    tracing::info!("Starting MapToPoster Rust server");
    tracing::info!("Loaded {} themes", themes::loader::load_themes(&config.themes_dir).len());

    // Create application state
    let state = Arc::new(AppState::new(config.clone()));

    // Build CORS layer
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    // Build the router
    let app = Router::new()
        // Health endpoints
        .route("/health", get(api::handlers::health::health_check))
        .route("/health/ready", get(api::handlers::health::readiness_check))
        // API routes
        .route("/api/themes", get(api::handlers::themes::list_themes))
        .route("/api/themes/:name", get(api::handlers::themes::get_theme))
        .route("/api/locations/search", get(api::handlers::locations::search_locations))
        .route("/api/posters", post(api::handlers::posters::create_poster))
        .route("/api/posters/:job_id", get(api::handlers::posters::get_poster_status))
        .route("/api/posters/:job_id/download", get(api::handlers::posters::download_poster))
        .route("/api/posters/:job_id/rerender", post(api::handlers::posters::rerender_poster))
        .route("/api/posters/:job_id/stream", get(api::handlers::jobs::stream_progress))
        // Also support /api/jobs path for frontend compatibility
        .route("/api/jobs/:job_id/stream", get(api::handlers::jobs::stream_progress))
        // Static files for generated posters
        .nest_service("/static", ServeDir::new(&config.static_dir))
        // Serve frontend
        .nest_service("/", ServeDir::new(&config.frontend_dir).append_index_html_on_directories(true))
        .layer(cors)
        .layer(TraceLayer::new_for_http())
        .with_state(state);

    // Start server
    let addr = SocketAddr::from(([0, 0, 0, 0], config.port));
    tracing::info!("Server listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
