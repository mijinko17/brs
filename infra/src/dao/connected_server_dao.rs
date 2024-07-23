use sea_orm::EntityTrait;
use util::{async_trait, error_handling::AppResult, new};

use crate::entity::connected_server;

use super::database_connection_factory::DatabaseConnectionFactory;

#[async_trait]
pub trait ConnectedServerDao {
    async fn save(&self, connected_servers: Vec<connected_server::ActiveModel>) -> AppResult<()>;
    async fn connected_servers(&self) -> AppResult<Vec<connected_server::Model>>;
    async fn delete_all(&self) -> AppResult<()>;
}

#[derive(new)]
pub struct ConnectedServerDaoImpl<T: DatabaseConnectionFactory> {
    connection_factory: T,
}

#[async_trait]
impl<T: DatabaseConnectionFactory + Sync> ConnectedServerDao for ConnectedServerDaoImpl<T> {
    async fn save(&self, connected_servers: Vec<connected_server::ActiveModel>) -> AppResult<()> {
        let db = self.connection_factory.connection().await?;
        connected_server::Entity::insert_many(connected_servers)
            .on_empty_do_nothing()
            .exec(&db)
            .await?;
        Ok(())
    }

    async fn connected_servers(&self) -> AppResult<Vec<connected_server::Model>> {
        let db = self.connection_factory.connection().await?;
        Ok(crate::entity::connected_server::Entity::find()
            .all(&db)
            .await?)
    }

    async fn delete_all(&self) -> AppResult<()> {
        let db = self.connection_factory.connection().await?;
        connected_server::Entity::delete_many().exec(&db).await?;
        Ok(())
    }
}
