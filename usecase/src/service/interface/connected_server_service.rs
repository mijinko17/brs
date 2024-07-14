use util::{async_trait, error_handling::AppResult};

use crate::output::connected_server_output::ConnectedServerOutput;

#[async_trait]
pub trait ConnectedServerService {
    async fn connected_servers(&self) -> AppResult<Vec<ConnectedServerOutput>>;
}
