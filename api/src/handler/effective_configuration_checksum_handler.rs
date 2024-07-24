use axum::{http::StatusCode, Json};
use controller::response::{
    effective_configuration_checksum_response::{
        EffectiveConfigurationChecksumResponse, EffectiveConfigurationChecksumWrapResponse,
    },
    rest_response::RestResponse,
};
use util::error_handling::AppResult;

pub const EFFECTIVE_CONFIGURATION_CHECKSUM_URL: &str =
    "/rest/running/zoning/effective-configuration/checksum";

pub async fn get_effective_configuration_checksum_handler() -> AppResult<(
    StatusCode,
    Json<RestResponse<EffectiveConfigurationChecksumWrapResponse>>,
)> {
    Ok((
        StatusCode::OK,
        Json(RestResponse::new(
            EffectiveConfigurationChecksumWrapResponse::new(
                EffectiveConfigurationChecksumResponse::new("checksum".to_string()),
            ),
        )),
    ))
}
