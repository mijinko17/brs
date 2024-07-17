use serde::Deserialize;

#[allow(dead_code)]
#[derive(Deserialize)]
pub struct SaveZoneConfigurationPayload {
    #[serde(rename = "checksum")]
    checksum: String,
}
