use serde::Deserialize;

#[derive(Deserialize)]
pub struct EnableEffectiveConfigurationPayload {
    #[serde(rename = "checksum")]
    checksum: String,
}
