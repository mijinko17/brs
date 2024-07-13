use std::{env::current_dir, future::IntoFuture, net::SocketAddr, vec};

use axum::{async_trait, debug_handler, http::StatusCode, response::Html, routing::get, Json, Router};
use axum_server::tls_rustls::RustlsConfig;
use controller::{
    controller::interface::zone_configuratin_controller::ZoneConfigurationController,
    response::zone_response::ZoneResponse,
};
use injection::zone_configuratin_controller;
// use injection::zone_configuratin_controller;
pub mod handler;

pub async fn start() {
    // build our application with a route
    // let x = handler2;
    let app = Router::new()
        .route("/", get(handler))
        .route("/zone", get(handler2));

    let config = RustlsConfig::from_pem_file(
        current_dir()
            .unwrap()
            .join("docker_files")
            .join("self_signed_certs")
            .join("cert.pem"),
        current_dir()
            .unwrap()
            .join("docker_files")
            .join("self_signed_certs")
            .join("key.pem"),
    )
    .await
    .unwrap();

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    axum_server::bind_rustls(addr, config)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn handler() -> Html<&'static str> {
    Html("<h1>Hello, World!</h1>")
}

async fn handler2() -> (StatusCode, Json<Vec<ZoneResponse>>) {
    // let b: Vec<ZoneResponse> = vec![ZoneResponse::new("hoge".to_string(), vec![])];
    let b: Vec<ZoneResponse> = zone_configuratin_controller().zones().await;
    // let b: Vec<ZoneResponse> = hoge().await;
    let a: Json<Vec<ZoneResponse>> = Json(b);
    (StatusCode::OK, a)
}

async fn hoge() -> Vec<ZoneResponse> {
    vec![]
}
