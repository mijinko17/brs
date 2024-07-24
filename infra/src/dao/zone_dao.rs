use sea_orm::{
    sea_query::IntoCondition, ActiveModelTrait, ColumnTrait, Condition, EntityTrait, ModelTrait,
    QueryFilter,
};
use util::{async_trait, error_handling::AppResult, new, Context};

use crate::entity::{prelude::Wwn, wwn, zone};

use super::database_connection_factory::DatabaseConnectionFactory;

pub struct DeleteZoneEntry(pub String);

#[async_trait]
pub trait ZoneDao {
    async fn zones(&self) -> AppResult<Vec<(zone::Model, Vec<wwn::Model>)>>;
    async fn zones_with_filter<F: IntoCondition + Send>(
        &self,
        confition: F,
    ) -> AppResult<Vec<(zone::Model, Vec<wwn::Model>)>>;
    async fn zones_by_name(
        &self,
        names: Vec<String>,
    ) -> AppResult<Vec<(zone::Model, Vec<wwn::Model>)>>;
    async fn update(&self, updates: Vec<zone::ActiveModel>) -> AppResult<()>;
    async fn save(&self, zone: zone::ActiveModel) -> AppResult<zone::Model>;
    async fn delete(&self, delete_zones: Vec<DeleteZoneEntry>) -> AppResult<()>;
    async fn delete_all(&self) -> AppResult<()>;
}

#[derive(new)]
pub struct ZoneDaoImpl<T: DatabaseConnectionFactory> {
    connection_factory: T,
}

#[async_trait]
impl<T: DatabaseConnectionFactory + Sync> ZoneDao for ZoneDaoImpl<T> {
    async fn zones(&self) -> AppResult<Vec<(zone::Model, Vec<wwn::Model>)>> {
        let db = self.connection_factory.connection().await?;
        let a = crate::entity::zone::Entity::find()
            .find_with_related(Wwn)
            .all(&db)
            .await?;
        Ok(a)
    }
    async fn zones_with_filter<F: IntoCondition + Send>(
        &self,
        confition: F,
    ) -> AppResult<Vec<(zone::Model, Vec<wwn::Model>)>> {
        let db = self.connection_factory.connection().await?;
        let result = crate::entity::zone::Entity::find()
            .filter(confition)
            .find_with_related(Wwn)
            .all(&db)
            .await?;
        Ok(result)
    }

    async fn update(&self, updates: Vec<zone::ActiveModel>) -> AppResult<()> {
        let db = self.connection_factory.connection().await?;
        for update in updates {
            update.update(&db).await?;
        }
        Ok(())
    }

    async fn zones_by_name(
        &self,
        names: Vec<String>,
    ) -> AppResult<Vec<(zone::Model, Vec<wwn::Model>)>> {
        let db = self.connection_factory.connection().await?;
        let result = crate::entity::zone::Entity::find()
            .filter(names.into_iter().fold(Condition::any(), |acc, cur| {
                acc.add(zone::Column::Name.eq(cur))
            }))
            .find_with_related(Wwn)
            .all(&db)
            .await?;
        Ok(result)
    }

    async fn save(&self, zone: zone::ActiveModel) -> AppResult<zone::Model> {
        let db = self.connection_factory.connection().await?;
        let model = zone::Entity::insert(zone).exec_with_returning(&db).await?;
        Ok(model)
    }

    async fn delete(&self, delete_zones: Vec<DeleteZoneEntry>) -> AppResult<()> {
        let db = self.connection_factory.connection().await?;
        let mut delete_targets = vec![];
        for delete_zone in delete_zones {
            let target = zone::Entity::find()
                .filter(zone::Column::Name.eq(delete_zone.0.clone()))
                .one(&db)
                .await?
                .context(format!("Not found zone {}", delete_zone.0))?;
            delete_targets.push(target);
        }
        for target in delete_targets {
            target.delete(&db).await?;
        }
        Ok(())
    }
    async fn delete_all(&self) -> AppResult<()> {
        let db = self.connection_factory.connection().await?;
        zone::Entity::delete_many().exec(&db).await?;
        Ok(())
    }
}
