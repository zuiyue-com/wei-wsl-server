use axum::Router;
use axum::routing::{post,get};
use tower_http::cors::{Any};

pub mod run;

pub fn routes() -> Router {
    Router::new()
        .route("/run", post(run::index))
        .route("/version", get(|| async { "wei-wsl-server" }))
        .layer(
            tower_http::cors::CorsLayer::new()
                .allow_origin("*".parse::<axum::http::HeaderValue>().unwrap())
                .allow_headers(Any)
                .allow_methods(Any),
        )
}
