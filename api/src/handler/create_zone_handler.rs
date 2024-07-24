use axum::{extract::Path, http::StatusCode, Json};
use controller::{
    controller::interface::zone_controller::ZoneController,
    payload::create_zone_payload::CreateZonePayload,
};
use injection::zone_controller;
use util::error_handling::AppResult;

pub const DEFINED_CONFIGURATION_ZONE_URL: &str =
    "/rest/running/zoning/defined-configuration/zone/zone-name/:zone_name";

pub async fn create_zone_handler(
    Path(zone_name): Path<String>,
    Json(payload): Json<CreateZonePayload>,
) -> AppResult<(StatusCode, ())> {
    zone_controller().create_zone(zone_name, payload).await?;
    Ok((StatusCode::CREATED, ()))
}
