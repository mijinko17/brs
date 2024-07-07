use std::{env::current_dir, net::SocketAddr};

use axum::{response::Html, routing::get, Router};
use axum_server::tls_rustls::RustlsConfig;

#[tokio::main]
async fn main() {
    // build our application with a route
    let app = Router::new().route("/", get(handler));

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

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    axum_server::bind_rustls(addr, config)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn handler() -> Html<&'static str> {
    Html("<h1>Hello, World!</h1>")
}
