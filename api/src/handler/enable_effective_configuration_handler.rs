use axum::{extract::Path, http::StatusCode, Json};
use controller::{
    controller::interface::zone_configuratin_controller::ZoneConfigurationController,
    payload::enable_effective_configuration_payload::EnableEffectiveConfigurationPayload,
};
use injection::zone_configuratin_controller;
use util::error_handling::AppResult;

pub const ENABLE_EFFECTIVE_CONFIGURATION_URL: &str =
    "/rest/running/zoning/effective-configuration/cfg-name/:cofig_name";

pub async fn enable_effective_configuration_handler(
    Path(config_name): Path<String>,
    _: Json<EnableEffectiveConfigurationPayload>,
) -> AppResult<(StatusCode, ())> {
    zone_configuratin_controller()
        .enable_zone_configuration(config_name)
        .await?;
    Ok((StatusCode::NO_CONTENT, ()))
}
