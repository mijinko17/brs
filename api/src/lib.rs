use axum::{
    response::Html,
    routing::{delete, get, patch, post},
    Router,
};
use axum_server::tls_rustls::RustlsConfig;
use handler::{
    create_zone_handler::{create_zone_handler, DEFINED_CONFIGURATION_ZONE_URL},
    defined_configuration_config_handler::{
        update_zone_configuration_member_handler, DEFINED_CONFIGURATION_CONFIG_URL,
    },
    defined_configuration_zone_name_handler::{
        delete_zone_handler, DEFINED_CONFIGURATION_ZONE_NAME_URL,
    },
    effective_configuration_action_handler::{
        save_zone_configuration_handler, EFFECTIVE_CONFIGURATION_ACTION_URL,
    },
    effective_configuration_checksum_handler::{
        get_effective_configuration_checksum_handler, EFFECTIVE_CONFIGURATION_CHECKSUM_URL,
    },
    effective_configuration_handler::{
        get_effective_configuration_handler, EFFECTIVE_CONFIGURATION_URL,
    },
    enable_effective_configuration_handler::{
        enable_effective_configuration_handler, ENABLE_EFFECTIVE_CONFIGURATION_URL,
    },
    fabric_switches_handler::{get_fabric_switches_handler, FABRIC_SWITCH_URL},
    fibrechannel_name_server_controller::{
        get_fibrechannel_name_server_handler, FIBRECHANNEL_NAME_SERVER_URL,
    },
    login_handler::{login_handler, LOGIN_URL},
    logout_handler::{logout_handler, LOGOUT_URL},
};
use std::{env::current_dir, net::SocketAddr};
pub mod handler;

pub async fn start() {
    let app = Router::new()
        .route("/", get(handler))
        .route("/zone", get(get_effective_configuration_handler))
        // .route("/fail", get(handler3))
        .route(LOGIN_URL, post(login_handler))
        .route(LOGOUT_URL, post(logout_handler))
        .route(FABRIC_SWITCH_URL, get(get_fabric_switches_handler))
        .route(
            FIBRECHANNEL_NAME_SERVER_URL,
            get(get_fibrechannel_name_server_handler),
        )
        .route(
            EFFECTIVE_CONFIGURATION_CHECKSUM_URL,
            get(get_effective_configuration_checksum_handler),
        )
        .route(
            EFFECTIVE_CONFIGURATION_URL,
            get(get_effective_configuration_handler),
        )
        .route(DEFINED_CONFIGURATION_ZONE_URL, patch(create_zone_handler))
        .route(
            DEFINED_CONFIGURATION_CONFIG_URL,
            patch(update_zone_configuration_member_handler),
        )
        .route(
            EFFECTIVE_CONFIGURATION_ACTION_URL,
            patch(save_zone_configuration_handler),
        )
        .route(
            ENABLE_EFFECTIVE_CONFIGURATION_URL,
            patch(enable_effective_configuration_handler),
        )
        .route(
            DEFINED_CONFIGURATION_ZONE_NAME_URL,
            delete(delete_zone_handler),
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
