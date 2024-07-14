use util::{async_trait, error_handling::AppResult, new};

use crate::{
    output::connected_server_output::ConnectedServerOutput,
    repository::connected_server_repository::ConnectedServerRepository,
    service::interface::connected_server_service::ConnectedServerService,
};

#[derive(new)]
pub struct ConnectedServerServiceImpl<T: ConnectedServerRepository> {
    connected_server_repository: T,
}

#[async_trait]
impl<T: ConnectedServerRepository + Sync> ConnectedServerService for ConnectedServerServiceImpl<T> {
    async fn connected_servers(&self) -> AppResult<Vec<ConnectedServerOutput>> {
        Ok(self
            .connected_server_repository
            .connected_servers()
            .await?
            .into_iter()
            .map(|connected_server| ConnectedServerOutput::new(connected_server.wwn))
            .collect())
    }
}
