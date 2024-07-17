use sea_orm::{Database, EntityTrait};
use util::{async_trait, error_handling::AppResult};

use crate::{entity::connected_server, DATABASE_URL};

#[async_trait]
pub trait ConnectedServerDao {
    async fn save(&self, connected_servers: Vec<connected_server::ActiveModel>) -> AppResult<()>;
    async fn delete_all(&self) -> AppResult<()>;
}

pub struct ConnectedServerDaoImpl;

#[async_trait]
impl ConnectedServerDao for ConnectedServerDaoImpl {
    async fn save(&self, connected_servers: Vec<connected_server::ActiveModel>) -> AppResult<()> {
        let db = Database::connect(DATABASE_URL).await?;
        connected_server::Entity::insert_many(connected_servers)
            .on_empty_do_nothing()
            .exec(&db)
            .await?;
        Ok(())
    }

    async fn delete_all(&self) -> AppResult<()> {
        let db = Database::connect(DATABASE_URL).await?;
        connected_server::Entity::delete_many().exec(&db).await?;
        Ok(())
    }
}
