use serde::Deserialize;

#[allow(dead_code)]
#[derive(Deserialize)]
pub struct EnableEffectiveConfigurationPayload {
    #[serde(rename = "checksum")]
    checksum: String,
}
