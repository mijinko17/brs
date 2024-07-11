use std::{env::current_dir, net::SocketAddr};

use axum::{response::Html, routing::get, Router};
use axum_server::tls_rustls::RustlsConfig;

use entities::post;
use futures::executor::block_on;
use sea_orm::{ActiveModelTrait, ActiveValue, Database, DbErr, EntityTrait, InsertResult};

// Change this according to your database implementation,
// or supply it as an environment variable.
// the whole database URL string follows the following format:
// "protocol://username:password@host:port/database"
// We put the database name (that last bit) in a separate variable simply for convenience.
const DATABASE_URL: &str = "sqlite:./piyo.db?mode=rwc";
const DB_NAME: &str = "bakeries_db";

mod entities;

async fn run() -> Result<(), DbErr> {
    let db = Database::connect(DATABASE_URL).await?;
    println!("aaa");
    let hoge = post::ActiveModel {
        text: ActiveValue::set("textxt".to_owned()),
        title: ActiveValue::set("titile".to_owned()),
        ..Default::default()
    };
    println!("{:?}", hoge.insert(&db).await);
    Ok(())
}

fn main() {
    if let Err(err) = block_on(run()) {
        panic!("{}", err);
    }
}

// #[tokio::main]
// async fn main() {
//     // build our application with a route
//     let app = Router::new().route("/", get(handler));
//
//     let config = RustlsConfig::from_pem_file(
//         current_dir()
//             .unwrap()
//             .join("self_signed_certs")
//             .join("cert.pem"),
//         current_dir()
//             .unwrap()
//             .join("self_signed_certs")
//             .join("key.pem"),
//     )
//     .await
//     .unwrap();
//
//     let addr = SocketAddr::from(([0, 0, 0, 0], 443));
//     axum_server::bind_rustls(addr, config)
//         .serve(app.into_make_service())
//         .await
//         .unwrap();
// }

async fn handler() -> Html<&'static str> {
    Html("<h1>Hello, World!</h1>")
}
