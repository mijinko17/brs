use domain::{
    entity::zone_configuration::ZoneConfiguration,
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
        let b = a.map(|model| ZoneConfiguration::new(model.name, vec![]));
        Ok(b)
    }
}
