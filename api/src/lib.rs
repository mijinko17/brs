use axum::{
    response::Html,
    routing::{get, post}, Router,
};
use axum_server::tls_rustls::RustlsConfig;
use handler::{
    fabric_switches_handler::{get_fabric_switches_handler, FABRIC_SWITCH_URL},
    get_effective_configuration_handler::{
        get_effective_configuration_handler, GET_EFFECTIVE_CONFIGURATION_URL,
    },
    login_handler::{login_handler, LOGIN_URL},
};
use std::{env::current_dir, net::SocketAddr};
pub mod handler;

pub async fn start() {
    // build our application with a route
    // let x = handler2;
    let app = Router::new()
        .route("/", get(handler))
        .route("/zone", get(get_effective_configuration_handler))
        // .route("/fail", get(handler3))
        .route(LOGIN_URL, post(login_handler))
        .route(FABRIC_SWITCH_URL, get(get_fabric_switches_handler))
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

// impl<E> From<E> for AppError
// where
//     E: Into<util::Error>,
// {
//     fn from(err: E) -> Self {
//         Self(err.into())
//     }
// }

// struct AppError(util::Error);
// impl IntoResponse for AppError {
//     fn into_response(self) -> axum::response::Response {
//         (StatusCode::BAD_REQUEST, "hoge").into_response()
//     }
// }

// impl IntoResponse for AppError {
//     fn into_response(self) -> axum::response::Response {
//         match self {
//             Self::Internal(e) => (StatusCode::INTERNAL_SERVER_ERROR, "internal"),
//             Self::BadRequest(e) => (StatusCode::BAD_REQUEST, "bad request"),
//         }
//         .into_response()
//     }
// }

// async fn hoge() -> Vec<ZoneResponse> {
//     vec![]
// }
