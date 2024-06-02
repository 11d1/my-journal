use axum::{routing::get, Router};

mod home;

use home::home;

pub async fn routes() -> Router {

    Router::new()
        .route("/", get(home))
}
