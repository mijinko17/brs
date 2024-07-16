use axum::{http::StatusCode, Json};
use controller::{
    controller::interface::zone_configuratin_controller::ZoneConfigurationController,
    response::{
        effective_configuration_response::EffectiveConfigurationWrapResponse,
        rest_response::RestResponse,
    },
};
use injection::zone_configuratin_controller;
use util::error_handling::AppResult;

pub const EFFECTIVE_CONFIGURATION_URL: &str = "/rest/running/brocade-zone/effective-configuration";

pub async fn get_effective_configuration_handler() -> AppResult<(
    StatusCode,
    Json<RestResponse<EffectiveConfigurationWrapResponse>>,
)> {
    let response = zone_configuratin_controller()
        .effective_configuration()
        .await?;
    Ok((StatusCode::OK, Json(response)))
}
