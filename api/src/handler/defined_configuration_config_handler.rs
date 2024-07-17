use axum::{extract::Path, http::StatusCode, Json};
use controller::payload::update_zone_configuration_member_payload::UpdateZoneConfigurationMemberWrapPayload;
use util::error_handling::AppResult;

pub const DEFINED_CONFIGURATION_CONFIG_URL: &str =
    "/rest/running/zoning/defined-configuration/cfg/cfg-name/:_cfg_name";

pub async fn update_zone_configuration_member_handler(
    Path(_cfg_name): Path<String>,
    _: Json<UpdateZoneConfigurationMemberWrapPayload>,
) -> AppResult<(StatusCode, ())> {
    Ok((StatusCode::NO_CONTENT, ()))
}
