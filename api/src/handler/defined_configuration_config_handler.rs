use axum::{http::StatusCode, Json};
use controller::payload::save_zone_configuration_payload::SaveZoneConfigurationPayload;
use util::error_handling::AppResult;

pub const DEFINED_CONFIGURATION_CONFIG_URL: &str =
    "/rest/running/brocade-zone/defined-configuration/cfg";

pub async fn update_zone_configuration_member_handler(
    _: Json<SaveZoneConfigurationPayload>,
) -> AppResult<(StatusCode, ())> {
    Ok((StatusCode::NO_CONTENT, ()))
}
