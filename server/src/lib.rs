use std::net::SocketAddr;

use axum::ServiceExt;
use axum_server::tls_rustls::RustlsConfig;
use clap::Parser;
use routes::routes;
use tower::Layer;
use tower_http::normalize_path::NormalizePathLayer;
use utils::clap::Opt;

mod utils;
mod routes;
mod tests;

pub async fn run() {
    let opt = Opt::parse();

    // Console logging
    if opt.log_level.to_lowercase().contains("y") {
        tracing_subscriber::fmt::init();
    }

    // Socket
    let sock_addr = SocketAddr::new(opt.addr, opt.port);

    // App
    let app = NormalizePathLayer::trim_trailing_slash()
        .layer(routes().await);

    // Try to TLS
    let tls_config = RustlsConfig::from_pem_file(
        opt.pem_cert_path, opt.pem_key_path)
            .await;
    
    log::info!("LISTENING on {}", &sock_addr);

    match tls_config {
        Ok(cfg) => {
            log::info!("--> TLS mode (https)");

            axum_server::bind_rustls(sock_addr, cfg)
                .serve(app.into_make_service())
                .await
                .expect("Failed to run the server");
        },
        Err(_) => {
            log::info!("--> NO TLS mode (http)");
            
            axum::Server::bind(&sock_addr)
                .serve(app.into_make_service())
                .await
                .expect("Failed to run the server");
        }
    }
}