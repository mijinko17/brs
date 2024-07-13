use axum::{http::StatusCode, Json};
use controller::{
    controller::interface::zone_configuratin_controller::ZoneConfigurationController,
    response::zone_response::ZoneResponse,
};
use injection::zone_configuratin_controller;

async fn handler2() -> (StatusCode, Json<Vec<ZoneResponse>>) {
    // let b: Vec<ZoneResponse> = vec![ZoneResponse::new("hoge".to_string(), vec![])];
    let b: Vec<ZoneResponse> = zone_configuratin_controller().zones().await;
    // let b: Vec<ZoneResponse> = hoge().await;
    let a: Json<Vec<ZoneResponse>> = Json(b);
    (StatusCode::OK, a)
}
