use util::{async_trait, error_handling::AppResult};

use crate::{
    input::{
        create_zone_configuration_input::CreateZoneConfigurationInput,
        enable_zone_configuration_input::EnableZoneConfigurationInput,
        modify_zone_configuration_member_input::ModifyZoneConfigurationMemberInput,
    },
    output::zone_configuration_output::ZoneConfigurationOutput,
};

#[async_trait]
pub trait ZoneConfigurationService {
    async fn effective_configuration(&self) -> AppResult<Option<ZoneConfigurationOutput>>;
    async fn modify_zone_configuration_member(
        &self,
        input: ModifyZoneConfigurationMemberInput,
    ) -> AppResult<()>;
    async fn create_zone_configuration(
        &self,
        input: CreateZoneConfigurationInput,
    ) -> AppResult<()>;
    async fn enable_zone_configuration(&self, input: EnableZoneConfigurationInput)
        -> AppResult<()>;
}
