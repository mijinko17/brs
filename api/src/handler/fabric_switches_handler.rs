use axum::{http::StatusCode, Json};
use controller::{
    controller::interface::fabric_switch_controller::FabricSwitchController,
    response::{fabric_switch_response::FabricSwitchWrapResponse, rest_response::RestResponse},
};
use injection::fabric_switch_controller;

pub const FABRIC_SWITCH_URL: &str = "/rest/running/brocade-fabric/fabric-switch";

pub async fn get_fabric_switches_handler(
) -> (StatusCode, Json<RestResponse<FabricSwitchWrapResponse>>) {
    let b: RestResponse<FabricSwitchWrapResponse> =
        fabric_switch_controller().fabric_switches().await;
    let a: Json<RestResponse<FabricSwitchWrapResponse>> = Json(b);
    (StatusCode::OK, a)
}
