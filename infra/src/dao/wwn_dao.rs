use sea_orm::EntityTrait;
use util::{async_trait, error_handling::AppResult, new};

use crate::entity::wwn;

use super::database_connection_factory::DatabaseConnectionFactory;

#[async_trait]
pub trait WwnDao {
    async fn save(&self, models: Vec<wwn::ActiveModel>) -> AppResult<()>;
}

#[derive(new)]
pub struct WwnDaoImpl<T: DatabaseConnectionFactory> {
    connection_factory: T,
}

#[async_trait]
impl<T: DatabaseConnectionFactory + Sync> WwnDao for WwnDaoImpl<T> {
    async fn save(&self, models: Vec<wwn::ActiveModel>) -> AppResult<()> {
        let db = self.connection_factory.connection().await?;
        wwn::Entity::insert_many(models)
            .on_empty_do_nothing()
            .exec(&db)
            .await?;
        Ok(())
    }
}
