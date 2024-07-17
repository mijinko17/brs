use axum::{extract::Path, http::StatusCode};
use controller::controller::interface::zone_controller::ZoneController;
use injection::zone_controller;
use util::error_handling::AppResult;

pub const DEFINED_CONFIGURATION_ZONE_NAME_URL: &str =
    "/rest/running/zoning/defined-configuration/zone/zone-name/:zone_name";

pub async fn delete_zone_handler(Path(zone_name): Path<String>) -> AppResult<(StatusCode, ())> {
    zone_controller().delete_zone(zone_name).await?;
    Ok((StatusCode::NO_CONTENT, ()))
}
