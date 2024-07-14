use std::vec;
use util::{async_trait, error_handling::AppResult, new};

use crate::{
    entity::{wwn::Wwn, zone::Zone},
    input::create_zones_input::CreateZonesInput,
    output::{
        wwn_output::WwnOutput, zone_configuration_output::ZoneConfigurationOutput,
        zone_output::ZoneOutput,
    },
    repository::zone_repository::ZoneRepository,
    service::interface::zone_service::ZoneService,
};

#[derive(new)]
pub struct ZoneServiceImpl<T>
where
    T: ZoneRepository,
{
    repository: T,
}

#[async_trait]
impl<T> ZoneService for ZoneServiceImpl<T>
where
    T: ZoneRepository + Sync,
{
    async fn create_zones(&self, input: CreateZonesInput) -> AppResult<()> {
        let zones = input
            .zone_inputs
            .into_iter()
            .map(|zone_input| {
                Zone::new(
                    zone_input.name,
                    zone_input
                        .members
                        .into_iter()
                        .map(|wwn_input| Wwn::new(wwn_input.value))
                        .collect(),
                )
            })
            .collect();
        self.repository.save(zones).await?;
        Ok(())
    }

    async fn zones(&self) -> AppResult<Vec<crate::output::zone_output::ZoneOutput>> {
        Ok(self
            .repository
            .zones()
            .await?
            .into_iter()
            .map(|zone| ZoneOutput::new(zone.name(), vec![]))
            .collect())
    }
    async fn effective_configuration(&self) -> AppResult<ZoneConfigurationOutput> {
        Ok(ZoneConfigurationOutput::new(
            "MainCfg".to_string(),
            self.repository
                .zones()
                .await?
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
        ))
    }
}
