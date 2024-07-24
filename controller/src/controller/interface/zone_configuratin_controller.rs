use util::{async_trait, error_handling::AppResult};

use crate::{
    payload::{
        create_zone_configuration_payload::CreateZoneConfigurationPayload,
        update_zone_configuration_member_payload::UpdateZoneConfigurationPayload,
    },
    response::{
        effective_configuration_response::EffectiveConfigurationWrapResponse,
        rest_response::RestResponse,
    },
};

#[async_trait]
pub trait ZoneConfigurationController {
    async fn effective_configuration(
        &self,
    ) -> AppResult<RestResponse<EffectiveConfigurationWrapResponse>>;
    async fn update_zone_configuration_member(
        &self,
        cfg_name: String,
        payload: UpdateZoneConfigurationPayload,
    ) -> AppResult<()>;
    async fn create_zone_configuration(
        &self,
        cfg_name: String,
        payload: CreateZoneConfigurationPayload,
    ) -> AppResult<()>;
    async fn enable_zone_configuration(&self, cfg_name: String) -> AppResult<()>;
}
