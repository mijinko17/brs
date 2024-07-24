use std::vec;

use domain::{
    input::{
        create_zone_configuration_input::CreateZoneConfigurationInput,
        enable_zone_configuration_input::EnableZoneConfigurationInput,
        modify_zone_configuration_member_input::ModifyZoneConfigurationMemberInput,
    },
    service::interface::zone_configuration_service::ZoneConfigurationService,
};
use util::{async_trait, error_handling::AppResult, new, wwn::format_wwn};

use crate::{
    controller::interface::zone_configuratin_controller::ZoneConfigurationController,
    payload::{
        create_zone_configuration_payload::CreateZoneConfigurationPayload,
        update_zone_configuration_member_payload::UpdateZoneConfigurationPayload,
    },
    response::{
        effective_configuration_response::{
            EffectiveConfigurationResponse, EffectiveConfigurationWrapResponse,
        },
        rest_response::RestResponse,
        zone_response::{ZoneMemberEntryResponse, ZoneResponse},
    },
};

#[derive(new)]
pub struct ZoneConfigurationControllerImpl<T>
where
    T: ZoneConfigurationService,
{
    zone_configuration_service: T,
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
            .zone_configuration_service
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

    async fn update_zone_configuration_member(
        &self,
        cfg_name: String,
        payload: UpdateZoneConfigurationPayload,
    ) -> AppResult<()> {
        self.zone_configuration_service
            .modify_zone_configuration_member(ModifyZoneConfigurationMemberInput::new(
                cfg_name,
                payload.member_zone.zone_name,
            ))
            .await?;
        Ok(())
    }

    async fn enable_zone_configuration(&self, cfg_name: String) -> AppResult<()> {
        self.zone_configuration_service
            .enable_zone_configuration(EnableZoneConfigurationInput::new(cfg_name))
            .await?;
        Ok(())
    }

    async fn create_zone_configuration(
        &self,
        cfg_name: String,
        payload: CreateZoneConfigurationPayload,
    ) -> AppResult<()> {
        self.zone_configuration_service
            .create_zone_configuration(CreateZoneConfigurationInput::new(
                cfg_name,
                payload.member_zone.zone_name,
            ))
            .await?;
        Ok(())
    }
}
