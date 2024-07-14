use axum::{
    http::StatusCode,
    response::Html,
    routing::{get, post},
    Json, Router,
};
use axum_server::tls_rustls::RustlsConfig;
use controller::{
    controller::interface::zone_configuratin_controller::ZoneConfigurationController,
    response::zone_response::ZoneResponse,
};
use handler::{
    get_effective_configuration_handler::{
        get_effective_configuration_handler, GET_EFFECTIVE_CONFIGURATION_URL,
    },
    login_handler::{login_handler, LOGIN_URL},
};
use injection::zone_configuratin_controller;
use std::{env::current_dir, net::SocketAddr};
// use injection::zone_configuratin_controller;
pub mod handler;

pub async fn start() {
    // build our application with a route
    // let x = handler2;
    let app = Router::new()
        .route("/", get(handler))
        .route("/zone", get(get_effective_configuration_handler))
        .route(LOGIN_URL, post(login_handler))
        .route(
            GET_EFFECTIVE_CONFIGURATION_URL,
            get(get_effective_configuration_handler),
        );

    let config = RustlsConfig::from_pem_file(
        current_dir()
            .unwrap()
            .join("self_signed_certs")
            .join("cert.pem"),
        current_dir()
            .unwrap()
            .join("self_signed_certs")
            .join("key.pem"),
    )
    .await
    .unwrap();

    let addr = SocketAddr::from(([0, 0, 0, 0], 443));
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
