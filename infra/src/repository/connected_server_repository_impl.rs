use domain::{
    entity::connected_server::ConnectedServer,
    repository::connected_server_repository::ConnectedServerRepository,
};
use util::{async_trait, error_handling::AppResult, new};

use crate::dao::connected_server_dao::ConnectedServerDao;

#[derive(new)]
pub struct ConnectedServerRepositoryImpl<T: ConnectedServerDao> {
    conected_server_dao: T,
}

#[async_trait]
impl<T: ConnectedServerDao + Sync> ConnectedServerRepository for ConnectedServerRepositoryImpl<T> {
    async fn connected_servers(&self) -> AppResult<Vec<ConnectedServer>> {
        Ok(self
            .conected_server_dao
            .connected_servers()
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
