use util::async_trait;

use crate::response::{
    effective_configuration_response::EffectiveConfigurationWrapResponse,
    rest_response::RestResponse, zone_response::ZoneResponse,
};

#[async_trait]
pub trait ZoneConfigurationController {
    async fn zones(&self) -> Vec<ZoneResponse>;
    async fn effective_configuration(&self) -> RestResponse<EffectiveConfigurationWrapResponse>;
}
