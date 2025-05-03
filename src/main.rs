mod errors;
mod proxy;
use crate::proxy::post_proxy;
use axum::{
    Router,
    routing::{get, post},
};
use tracing_subscriber::fmt;

#[tokio::main]
async fn main() {
    fmt().with_target(false).compact().init();

    // build our application with a single route
    let app = Router::new()
        .route("/healthy", get(is_healthy))
        .route("/proxy", post(post_proxy));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn is_healthy() -> &'static str {
    "OK"
}
