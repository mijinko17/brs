use util::{async_trait, error_handling::AppResult, new, Context};

use crate::{
    input::{
        create_zone_configuration_input::CreateZoneConfigurationInput,
        enable_zone_configuration_input::EnableZoneConfigurationInput,
        modify_zone_configuration_member_input::ModifyZoneConfigurationMemberInput,
    },
    output::{
        wwn_output::WwnOutput, zone_configuration_output::ZoneConfigurationOutput,
        zone_output::ZoneOutput,
    },
    repository::{
        zone_configuration_repository::ZoneConfigurationRepository, zone_repository::ZoneRepository,
    },
    service::interface::zone_configuration_service::ZoneConfigurationService,
};

#[derive(new)]
pub struct ZoneConfigurationServiceImpl<T: ZoneConfigurationRepository, U: ZoneRepository> {
    zone_config_repository: T,
    zone_repository: U,
}

#[async_trait]
impl<T: ZoneConfigurationRepository + Sync, U: ZoneRepository + Sync> ZoneConfigurationService
    for ZoneConfigurationServiceImpl<T, U>
{
    async fn effective_configuration(&self) -> AppResult<Option<ZoneConfigurationOutput>> {
        let a = self
            .zone_config_repository
            .effective_configuration()
            .await?;
        let b = a.map(|(_, zone_configuration)| {
            ZoneConfigurationOutput::new(
                zone_configuration.name,
                zone_configuration
                    .zones
                    .into_iter()
                    .map(|zone| {
                        ZoneOutput::new(
                            zone.name(),
                            zone.members()
                                .into_iter()
                                .map(|member| WwnOutput::new(member.value()))
                                .collect(),
                        )
                    })
                    .collect(),
            )
        });
        Ok(b)
    }

    async fn modify_zone_configuration_member(
        &self,
        input: ModifyZoneConfigurationMemberInput,
    ) -> AppResult<()> {
        let (config_identifier, _) = self
            .zone_config_repository
            .zone_configuration_by_name(input.config_name)
            .await?
            .context("Configuration not found.")?;
        let target_zone_identifiers = self
            .zone_repository
            .zones_by_name(input.members)
            .await?
            .into_iter()
            .map(|(zone_identifier, _)| zone_identifier)
            .collect();
        self.zone_config_repository
            .update_zone_member(config_identifier, target_zone_identifiers)
            .await?;
        Ok(())
    }

    async fn create_zone_configuration(
        &self,
        input: CreateZoneConfigurationInput,
    ) -> AppResult<()> {
        let target_zone_identifiers = self
            .zone_repository
            .zones_by_name(input.members)
            .await?
            .into_iter()
            .map(|(zone_identifier, _)| zone_identifier)
            .collect();
        self.zone_config_repository
            .create(input.config_name, target_zone_identifiers)
            .await?;
        Ok(())
    }

    async fn enable_zone_configuration(
        &self,
        input: EnableZoneConfigurationInput,
    ) -> AppResult<()> {
        let (target_identifier, _) = self
            .zone_config_repository
            .zone_configuration_by_name(input.0)
            .await?
            .context("Zone configuration not found.")?;
        self.zone_config_repository
            .enable_zone_configuration(target_identifier)
            .await?;
        Ok(())
    }
}
