use axum::{extract::Path, http::StatusCode, Json};
use controller::payload::enable_effective_configuration_payload::EnableEffectiveConfigurationPayload;
use util::error_handling::AppResult;

pub const ENABLE_EFFECTIVE_CONFIGURATION_URL: &str =
    "/rest/running/brocade-zone/effective-configuration/cfg-name/:_cofig_name";

pub async fn enable_effective_configuration_handler(
    Path(_config_name): Path<String>,
    _: Json<EnableEffectiveConfigurationPayload>,
) -> AppResult<(StatusCode, ())> {
    Ok((StatusCode::NO_CONTENT, ()))
}
