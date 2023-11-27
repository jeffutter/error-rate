use axum::{
    body::Body,
    extract::Path,
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::get,
    Router,
};
use rand::prelude::*;

async fn handler(Path(percent): Path<u32>) -> Response {
    let mut rng = rand::thread_rng();
    if rng.gen_ratio(percent, 100) {
        tracing::info!("Error on {} percent", percent);
        (StatusCode::INTERNAL_SERVER_ERROR, Body::empty()).into_response()
    } else {
        tracing::info!("OK on {} percent", percent);
        (StatusCode::OK, Body::empty()).into_response()
    }
}

#[tokio::main]
async fn main() {
    use tracing_subscriber::{
        filter::LevelFilter, prelude::__tracing_subscriber_SubscriberExt, util::SubscriberInitExt,
        EnvFilter, Layer,
    };
    let default_filter = EnvFilter::builder()
        .with_default_directive(LevelFilter::INFO.into())
        .from_env_lossy();
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer().with_filter(default_filter))
        .init();

    let app = Router::new()
        .route("/:percent", get(handler))
        .route(
            "/healthz",
            get(|| async { (StatusCode::OK, Body::empty()).into_response() }),
        )
        .route(
            "/livez",
            get(|| async { (StatusCode::OK, Body::empty()).into_response() }),
        )
        .route(
            "/readyz",
            get(|| async { (StatusCode::OK, Body::empty()).into_response() }),
        )
        .layer(tower_http::trace::TraceLayer::new_for_http());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Server Started");
    axum::serve(listener, app).await.unwrap();
}
