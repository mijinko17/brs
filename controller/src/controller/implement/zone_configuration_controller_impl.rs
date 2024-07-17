use usecase::service::interface::zone_service::ZoneService;
use util::{async_trait, error_handling::AppResult, wwn::format_wwn};

use crate::{
    controller::interface::zone_configuratin_controller::ZoneConfigurationController,
    response::{
        effective_configuration_response::{
            EffectiveConfigurationResponse, EffectiveConfigurationWrapResponse,
        },
        rest_response::RestResponse,
        zone_response::{ZoneMemberEntryResponse, ZoneResponse},
    },
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
    async fn effective_configuration(
        &self,
    ) -> AppResult<RestResponse<EffectiveConfigurationWrapResponse>> {
        let zones = self
            .zone_service
            .effective_configuration()
            .await?
            .zones
            .into_iter()
            .map(|zone_output| {
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
            })
            .collect::<Vec<_>>();
        Ok(RestResponse::new(EffectiveConfigurationWrapResponse::new(
            EffectiveConfigurationResponse::new(
                Some("nice-cfg".to_string()),
                zones,
                "checksum".to_string(),
            ),
        )))
    }
}
