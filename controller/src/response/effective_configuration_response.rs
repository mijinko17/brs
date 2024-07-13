use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct EffectiveConfigurationResponse {
    checksum: String,
}

#[derive(Debug, Serialize)]
pub struct EffectiveConfigurationWrapResponse {
    effective_configuration: EffectiveConfigurationResponse,
}
