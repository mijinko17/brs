use serde::Serialize;
use util::new;

use super::zone_response::ZoneResponse;

#[derive(Debug, Serialize, new)]
pub struct EffectiveConfigurationResponse {
    #[serde(rename = "checksum")]
    pub checksum: String,
    #[serde(rename = "enabled-zone")]
    pub enabled_zone: Vec<ZoneResponse>,
}

#[derive(Debug, Serialize, new)]
pub struct EffectiveConfigurationWrapResponse {
    #[serde(rename = "effective-configuration")]
    pub effective_configuration: EffectiveConfigurationResponse,
}
