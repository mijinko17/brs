use usecase::service::interface::zone_service::ZoneService;
use util::async_trait;

use crate::{
    controller::interface::zone_configuratin_controller::ZoneConfigurationController,
    response::{
        effective_configuration_response::{
            EffectiveConfigurationResponse, EffectiveConfigurationWrapResponse,
        },
        rest_response::RestResponse,
        zone_response::{ZoneMemberEntryResponse, ZoneResponse},
    },
    util::wwn::format_wwn,
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
            .map(|zone_output| {
                ZoneResponse::new(zone_output.name, ZoneMemberEntryResponse::new(vec![]))
            })
            .collect()
    }

    async fn effective_configuration(&self) -> RestResponse<EffectiveConfigurationWrapResponse> {
        let a = self.zone_service.effective_configuration().await;
        let b = a.zones;
        let c = b.into_iter().map(|zone_output| {
            ZoneResponse::new(
                zone_output.name,
                ZoneMemberEntryResponse::new(
                    zone_output
                        .members
                        .into_iter()
                        .map(|member| format_wwn(member.value))
                        .collect(),
                ),
            )
        });
        let d = EffectiveConfigurationResponse::new("checksum".to_string(), c.collect());
        let e = EffectiveConfigurationWrapResponse::new(d);
        RestResponse::new(e)
        // let z=EffectiveConfigurationResponse::new("checksum", )
    }
}
