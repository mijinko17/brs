use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter};
use util::{async_trait, error_handling::AppResult, new};

use crate::entity::{prelude::Wwn, wwn, zone, zone_configuration};

use super::database_connection_factory::DatabaseConnectionFactory;

#[async_trait]
pub trait ZoneConfigurationDao {
    async fn zone_configurations(&self) -> AppResult<Vec<zone_configuration::Model>>;
    async fn effective_configuration(
        &self,
    ) -> AppResult<
        Option<(
            zone_configuration::Model,
            Vec<(zone::Model, Vec<wwn::Model>)>,
        )>,
    >;

    async fn zone_configuration_by_name(
        &self,
        name: String,
    ) -> AppResult<
        Option<(
            zone_configuration::Model,
            Vec<(zone::Model, Vec<wwn::Model>)>,
        )>,
    >;

    async fn save(
        &self,
        model: zone_configuration::ActiveModel,
    ) -> AppResult<zone_configuration::Model>;

    async fn update(&self, updates: Vec<zone_configuration::ActiveModel>) -> AppResult<()>;
}

#[derive(new)]
pub struct ZoneConfigurationDaoImpl<T: DatabaseConnectionFactory> {
    connection_factory: T,
}

#[async_trait]
impl<T: DatabaseConnectionFactory + Sync> ZoneConfigurationDao for ZoneConfigurationDaoImpl<T> {
    async fn zone_configurations(&self) -> AppResult<Vec<zone_configuration::Model>> {
        let db = self.connection_factory.connection().await?;
        Ok(crate::entity::zone_configuration::Entity::find()
            .all(&db)
            .await?)
    }
    async fn effective_configuration(
        &self,
    ) -> AppResult<
        Option<(
            zone_configuration::Model,
            Vec<(zone::Model, Vec<wwn::Model>)>,
        )>,
    > {
        let db = self.connection_factory.connection().await?;
        let result = crate::entity::zone_configuration::Entity::find()
            .filter(zone_configuration::Column::IsEffective.eq(true))
            .one(&db)
            .await?;
        match result {
            Some(zone_configuration) => {
                let id = zone_configuration.id;
                let tuple = (
                    zone_configuration,
                    crate::entity::zone::Entity::find()
                        .filter(zone::Column::CfgId.eq(id))
                        .find_with_related(Wwn)
                        .all(&db)
                        .await?,
                );
                Ok(Some(tuple))
            }
            None => Ok(None),
        }
    }

    async fn zone_configuration_by_name(
        &self,
        name: String,
    ) -> AppResult<
        Option<(
            zone_configuration::Model,
            Vec<(zone::Model, Vec<wwn::Model>)>,
        )>,
    > {
        let db = self.connection_factory.connection().await?;
        let result = crate::entity::zone_configuration::Entity::find()
            .filter(zone_configuration::Column::Name.eq(name))
            .one(&db)
            .await?;
        match result {
            Some(zone_configuration) => {
                let id = zone_configuration.id;
                let tuple = (
                    zone_configuration,
                    crate::entity::zone::Entity::find()
                        .filter(zone::Column::CfgId.eq(id))
                        .find_with_related(Wwn)
                        .all(&db)
                        .await?,
                );
                Ok(Some(tuple))
            }
            None => Ok(None),
        }
    }

    async fn save(
        &self,
        model: zone_configuration::ActiveModel,
    ) -> AppResult<zone_configuration::Model> {
        let db = self.connection_factory.connection().await?;
        let result = zone_configuration::Entity::insert(model)
            .exec_with_returning(&db)
            .await?;
        Ok(result)
    }

    async fn update(&self, updates: Vec<zone_configuration::ActiveModel>) -> AppResult<()> {
        let db = self.connection_factory.connection().await?;
        for update in updates {
            update.update(&db).await?;
        }
        Ok(())
    }
}
