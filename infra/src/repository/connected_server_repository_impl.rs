use sea_orm::{Database, EntityTrait};
use usecase::{
    entity::connected_server::ConnectedServer,
    repository::connected_server_repository::ConnectedServerRepository,
};
use util::{async_trait, error_handling::AppResult};

use crate::DATABASE_URL;

pub struct ConnectedServerRepositoryImpl;

#[async_trait]
impl ConnectedServerRepository for ConnectedServerRepositoryImpl {
    async fn connected_servers(&self) -> AppResult<Vec<ConnectedServer>> {
        let db = Database::connect(DATABASE_URL).await?;
        Ok(crate::entity::connected_server::Entity::find()
            .all(&db)
            .await?
            .into_iter()
            .map(|model| {
                ConnectedServer::new([
                    model.v0, model.v1, model.v2, model.v3, model.v4, model.v5, model.v6, model.v7,
                ])
            })
            .collect())
    }
}
