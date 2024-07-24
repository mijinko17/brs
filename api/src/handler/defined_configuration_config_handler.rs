use axum::{extract::Path, http::StatusCode, Json};
use controller::{
    controller::interface::zone_configuratin_controller::ZoneConfigurationController,
    payload::{
        create_zone_configuration_payload::CreateZoneConfigurationPayload,
        update_zone_configuration_member_payload::UpdateZoneConfigurationPayload,
    },
};
use injection::zone_configuratin_controller;
use util::error_handling::AppResult;

pub const DEFINED_CONFIGURATION_CONFIG_URL: &str =
    "/rest/running/zoning/defined-configuration/cfg/cfg-name/:cfg_name";

pub async fn update_zone_configuration_member_handler(
    Path(cfg_name): Path<String>,
    Json(payload): Json<UpdateZoneConfigurationPayload>,
) -> AppResult<(StatusCode, ())> {
    zone_configuratin_controller()
        .update_zone_configuration_member(cfg_name, payload)
        .await?;
    Ok((StatusCode::NO_CONTENT, ()))
}

pub async fn create_zone_configuration_handler(
    Path(cfg_name): Path<String>,
    Json(payload): Json<CreateZoneConfigurationPayload>,
) -> AppResult<(StatusCode, ())> {
    zone_configuratin_controller()
        .create_zone_configuration(cfg_name, payload)
        .await?;
    Ok((StatusCode::NO_CONTENT, ()))
}
