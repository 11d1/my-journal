use axum::{routing::get, Router};


pub async fn routes() -> Router {

    Router::new()
        .route("/", get(hello))
}

pub async fn hello() -> String {
    "Init".to_owned()
}