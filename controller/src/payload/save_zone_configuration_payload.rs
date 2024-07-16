use serde::Deserialize;

#[derive(Deserialize)]
pub struct SaveZoneConfigurationPayload {
    #[serde(rename = "checksum")]
    checksum: String,
}
