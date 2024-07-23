use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
use util::{async_trait, error_handling::AppResult, new};

use crate::entity::zone_configuration;

use super::database_connection_factory::DatabaseConnectionFactory;

#[async_trait]
pub trait ZoneConfigurationDao {
    async fn effective_configuration(&self) -> AppResult<Option<zone_configuration::Model>>;
}

#[derive(new)]
pub struct ZoneConfigurationDaoImpl<T: DatabaseConnectionFactory> {
    connection_factory: T,
}

#[async_trait]
impl<T: DatabaseConnectionFactory + Sync> ZoneConfigurationDao for ZoneConfigurationDaoImpl<T> {
    async fn effective_configuration(&self) -> AppResult<Option<zone_configuration::Model>> {
        let db = self.connection_factory.connection().await?;
        let result = crate::entity::zone_configuration::Entity::find()
            .filter(zone_configuration::Column::IsEffective.eq(true))
            .one(&db)
            .await?;
        Ok(result)
    }
}
