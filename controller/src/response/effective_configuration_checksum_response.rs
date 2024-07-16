use serde::Serialize;
use util::new;

#[derive(Serialize, new)]
pub struct EffectiveConfigurationChecksumResponse {
    #[serde(rename = "checksum")]
    checksum: String,
}
