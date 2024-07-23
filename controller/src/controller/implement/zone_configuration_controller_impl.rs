use std::vec;

use domain::service::interface::zone_configuration_service::ZoneConfigurationService;
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
    T: ZoneConfigurationService,
{
    zone_service: T,
}

impl<T> ZoneConfigurationControllerImpl<T>
where
    T: ZoneConfigurationService,
{
    pub fn new(zone_service: T) -> Self {
        Self { zone_service }
    }
}

#[async_trait]
impl<T> ZoneConfigurationController for ZoneConfigurationControllerImpl<T>
where
    T: ZoneConfigurationService + Sync,
{
    async fn effective_configuration(
        &self,
    ) -> AppResult<RestResponse<EffectiveConfigurationWrapResponse>> {
        let effective_configuration_response = self
            .zone_service
            .effective_configuration()
            .await?
            .map(|output| {
                let zones = output
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
                EffectiveConfigurationResponse::new(
                    Some(output.name),
                    zones,
                    "checksum".to_string(),
                )
            })
            .unwrap_or(EffectiveConfigurationResponse::new(
                None,
                vec![],
                "checksum".to_string(),
            ));
        Ok(RestResponse::new(EffectiveConfigurationWrapResponse::new(
            effective_configuration_response,
        )))
    }
}
