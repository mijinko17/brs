use usecase::service::interface::zone_service::ZoneService;
use util::async_trait;

use crate::{
    controller::interface::zone_configuratin_controller::ZoneConfigurationController,
    response::{effective_configuration_response::EffectiveConfigurationWrapResponse, rest_response::RestResponse, zone_response::ZoneResponse},
};

pub struct ZoneConfigurationControllerImpl<T>
where
    T: ZoneService,
{
    zone_service: T,
}

impl<T> ZoneConfigurationControllerImpl<T>
where
    T: ZoneService,
{
    pub fn new(zone_service: T) -> Self {
        Self { zone_service }
    }
}

#[async_trait]
impl<T> ZoneConfigurationController for ZoneConfigurationControllerImpl<T>
where
    T: ZoneService + Sync,
{
    async fn zones(&self) -> Vec<ZoneResponse> {
        self.zone_service
            .zones()
            .await
            .into_iter()
            .map(|zone_output| ZoneResponse::new(zone_output.name, vec![]))
            .collect()
    }

    async fn effective_configuration(&self) -> RestResponse<EffectiveConfigurationWrapResponse> {
        todo!()
    }
}
