use util::{async_trait, error_handling::AppResult};

use crate::entity::connected_server::ConnectedServer;

#[async_trait]
pub trait ConnectedServerRepository {
    async fn connected_servers(&self) -> AppResult<Vec<ConnectedServer>>;
}
