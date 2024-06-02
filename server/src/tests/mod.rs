use axum::{http::StatusCode, routing::get, Router};
use axum_test::TestServer;

use crate::utils::app_error::AppError;

pub async fn fail_route() -> Result<(), AppError> {
    Err(AppError::new(StatusCode::IM_A_TEAPOT, "I'm a teapot"))
}

#[tokio::test]
async fn should_fail() {
    // Build an application with a route
    let app = Router::new()
        .route("/test", get(fail_route));

    // Run the application for testing
    let server = TestServer::new(app).unwrap();

    // Get the request
    let response = server
        .get("/test")
        .await;

    assert_eq!(response.status_code(), 200);
}