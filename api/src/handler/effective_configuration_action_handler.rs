use axum::{http::StatusCode, Json};
use controller::payload::save_zone_configuration_payload::SaveZoneConfigurationPayload;
use util::error_handling::AppResult;

pub const EFFECTIVE_CONFIGURATION_ACTION_URL: &str =
    "/rest/running/zoning/effective-configuration/cfg-action/1";

pub async fn save_zone_configuration_handler(
    _: Json<SaveZoneConfigurationPayload>,
) -> AppResult<(StatusCode, ())> {
    Ok((StatusCode::NO_CONTENT, ()))
}
