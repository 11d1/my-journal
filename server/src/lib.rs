mod utils;
mod routes;

use routes::routes;
use utils::clap::Opt;

use std::net::SocketAddr;

use clap::Parser;
use tower::Layer;
use tower_http::normalize_path::NormalizePathLayer;
use axum::ServiceExt;
use axum_server::tls_rustls::RustlsConfig;


pub async fn run() {
    let opt = Opt::parse();

    // Console logging
    if opt.log_level.to_lowercase().contains("y") {
        tracing_subscriber::fmt::init();
    }

    // Socket
    let sock_addr = SocketAddr::new(
        opt.addr.parse().unwrap(),
        opt.port.parse().unwrap());

    // App
    let app = NormalizePathLayer::trim_trailing_slash()
        .layer(routes().await);

    // TLS
    let tls_config = RustlsConfig::from_pem_file(
            opt.pem_cert,
            opt.pem_key)
                .await
                .unwrap();


    log::info!("LISTENING on {}", &sock_addr);

    axum_server::bind_rustls(sock_addr, tls_config)
        .serve(app.into_make_service())
        .await
        .unwrap();
}