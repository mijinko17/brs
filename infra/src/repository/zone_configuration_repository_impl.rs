use domain::{
    entity::{zone::Zone, zone_configuration::ZoneConfiguration},
    repository::zone_configuration_repository::ZoneConfigurationRepository,
};
use util::{async_trait, error_handling::AppResult, new};

use crate::dao::zone_configuration_dao::ZoneConfigurationDao;

#[derive(new)]
pub struct ZoneConfigurationRepositoryImpl<T: ZoneConfigurationDao> {
    zone_configuration_dao: T,
}

#[async_trait]
impl<T: ZoneConfigurationDao + Sync> ZoneConfigurationRepository
    for ZoneConfigurationRepositoryImpl<T>
{
    async fn effective_configuration(&self) -> AppResult<Option<ZoneConfiguration>> {
        let a = self
            .zone_configuration_dao
            .effective_configuration()
            .await?;
        let b = a.map(|(config, zone_with_wwn)| {
            ZoneConfiguration::new(
                config.name,
                zone_with_wwn
                    .into_iter()
                    .map(|(zone, wwns)| {
                        Zone::new(
                            zone.name,
                            wwns.into_iter()
                                .map(|wwn| {
                                    domain::entity::wwn::Wwn::new([
                                        wwn.v0, wwn.v1, wwn.v2, wwn.v3, wwn.v4, wwn.v5, wwn.v6,
                                        wwn.v7,
                                    ])
                                })
                                .collect(),
                        )
                    })
                    .collect(),
            )
        });
        Ok(b)
    }
}
