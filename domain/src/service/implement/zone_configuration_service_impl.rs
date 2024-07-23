use util::{async_trait, error_handling::AppResult, new};

use crate::{
    output::{
        wwn_output::WwnOutput, zone_configuration_output::ZoneConfigurationOutput,
        zone_output::ZoneOutput,
    },
    repository::zone_configuration_repository::ZoneConfigurationRepository,
    service::interface::zone_configuration_service::ZoneConfigurationService,
};

#[derive(new)]
pub struct ZoneConfigurationServiceImpl<T: ZoneConfigurationRepository> {
    zone_config_repository: T,
}

#[async_trait]
impl<T: ZoneConfigurationRepository + Sync> ZoneConfigurationService
    for ZoneConfigurationServiceImpl<T>
{
    async fn effective_configuration(&self) -> AppResult<Option<ZoneConfigurationOutput>> {
        let a = self
            .zone_config_repository
            .effective_configuration()
            .await?;
        let b = a.map(|entity| {
            ZoneConfigurationOutput::new(
                entity.name,
                entity
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
}
