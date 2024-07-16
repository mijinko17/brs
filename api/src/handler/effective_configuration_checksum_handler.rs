use axum::{http::StatusCode, Json};
use controller::response::effective_configuration_checksum_response::EffectiveConfigurationChecksumResponse;
use util::error_handling::AppResult;

pub const EFFECTIVE_CONFIGURATION_CHECKSUM_URL: &str =
    "/rest/running/brocade-zone/effective-configuration/checksum";

pub async fn get_effective_configuration_checksum_handler(
) -> AppResult<(StatusCode, Json<EffectiveConfigurationChecksumResponse>)> {
    Ok((
        StatusCode::OK,
        Json(EffectiveConfigurationChecksumResponse::new(
            "checksum".to_string(),
        )),
    ))
}
