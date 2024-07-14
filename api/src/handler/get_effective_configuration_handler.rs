use axum::{http::StatusCode, Json};
use controller::{
    controller::interface::zone_configuratin_controller::ZoneConfigurationController,
    response::{
        effective_configuration_response::EffectiveConfigurationWrapResponse,
        rest_response::RestResponse, zone_response::ZoneResponse,
    },
};
use injection::zone_configuratin_controller;

pub const GET_EFFECTIVE_CONFIGURATION_URL: &str =
    "/rest/running/brocade-zone/effective-configuration";

pub async fn get_effective_configuration_handler() -> (
    StatusCode,
    Json<RestResponse<EffectiveConfigurationWrapResponse>>,
) {
    let b: RestResponse<EffectiveConfigurationWrapResponse> = zone_configuratin_controller()
        .effective_configuration()
        .await;
    let a: Json<RestResponse<EffectiveConfigurationWrapResponse>> = Json(b);
    (StatusCode::OK, a)
}
