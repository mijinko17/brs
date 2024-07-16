use sea_orm::{Database, EntityTrait};
use util::{async_trait, error_handling::AppResult};

use crate::{entity::connected_server, DATABASE_URL};

#[async_trait]
pub trait ConnectedServerDao {
    async fn save(&self, connected_servers: Vec<connected_server::ActiveModel>) -> AppResult<()>;
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
}
