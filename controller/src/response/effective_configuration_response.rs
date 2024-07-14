use serde::Serialize;
use util::new;

use super::zone_response::ZoneResponse;

#[derive(Debug, Serialize, new)]
pub struct EffectiveConfigurationResponse {
    pub checksum: String,
    pub enabled_zone: Vec<ZoneResponse>,
}

#[derive(Debug, Serialize, new)]
pub struct EffectiveConfigurationWrapResponse {
    pub effective_configuration: EffectiveConfigurationResponse,
}
