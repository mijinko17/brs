use axum::{http::StatusCode, Json};
use controller::{
    controller::interface::zone_controller::ZoneController,
    payload::create_zone_payload::CreateZoneWrapPayload,
};
use injection::zone_controller;
use util::error_handling::AppResult;

pub const DEFINED_CONFIGURATION_ZONE_URL: &str =
    "/rest/running/brocade-zone/defined-configuration/zone";

pub async fn create_zone_handler(
    Json(CreateZoneWrapPayload { zone: payload }): Json<CreateZoneWrapPayload>,
) -> AppResult<(StatusCode, ())> {
    zone_controller().create_zone(payload).await?;
    Ok((StatusCode::CREATED, ()))
}
