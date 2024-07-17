use serde::Serialize;
use util::new;

use super::zone_response::ZoneResponse;

#[derive(Debug, Serialize, new)]
pub struct EffectiveConfigurationResponse {
    #[serde(rename = "cfg-name")]
    pub config_name: Option<String>,
    #[serde(rename = "enabled-zone")]
    pub enabled_zone: Vec<ZoneResponse>,
    #[serde(rename = "checksum")]
    pub checksum: String,
}

#[derive(Debug, Serialize, new)]
pub struct EffectiveConfigurationWrapResponse {
    #[serde(rename = "effective-configuration")]
    pub effective_configuration: EffectiveConfigurationResponse,
}
