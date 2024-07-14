use axum::{http::StatusCode, Json};
use controller::{
    controller::interface::fibrechannel_name_server_controller::FibrechannelNameServerController,
    response::{
        connected_server_response::FibrechannelNameServerWrapResponse, rest_response::RestResponse,
    },
};
use injection::fibrechannel_name_server_controller;
use util::error_handling::AppResult;

pub const FIBRECHANNEL_NAME_SERVER_URL: &str =
    "/rest/running/brocade-name-server/fibrechannel-name-server";

pub async fn get_fibrechannel_name_server_handler() -> AppResult<(
    StatusCode,
    Json<RestResponse<FibrechannelNameServerWrapResponse>>,
)> {
    let response = fibrechannel_name_server_controller()
        .fibrechannel_name_server()
        .await?;
    Ok((StatusCode::OK, Json(response)))
}
