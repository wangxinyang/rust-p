use std::sync::Arc;

use arc_swap::ArcSwap;
use arc_swap_coding::{Config, FeaturesConfig};
use axum::{
    response::IntoResponse,
    routing::{get, post},
    Extension, Router,
};
use tracing::info;

type FeaturesParams = Arc<ArcSwap<FeaturesConfig>>;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    let config = Config::load().await;
    let state = Arc::new(ArcSwap::new(Arc::new(config.features)));
    let app = Router::new()
        .route("/", get(get_handler))
        .route("/reload", post(reload_handler))
        .layer(Extension(state));

    let addr = config.network.into();
    info!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn get_handler(Extension(state): Extension<FeaturesParams>) -> impl IntoResponse {
    let params = state.load();
    format!(
        "font_size is: {}, font_color is: {}",
        params.font_size, params.font_color
    )
}

async fn reload_handler(Extension(state): Extension<FeaturesParams>) -> impl IntoResponse {
    let config = Config::load().await;
    state.store(Arc::new(config.features));
    "Reload... "
}
