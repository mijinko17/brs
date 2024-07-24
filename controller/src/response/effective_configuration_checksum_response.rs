use serde::Serialize;
use util::new;

#[derive(Serialize, new)]
pub struct EffectiveConfigurationChecksumWrapResponse {
    #[serde(rename = "effective-configuration")]
    effective_configuration: EffectiveConfigurationChecksumResponse,
}

#[derive(Serialize, new)]
pub struct EffectiveConfigurationChecksumResponse {
    #[serde(rename = "checksum")]
    checksum: String,
}
