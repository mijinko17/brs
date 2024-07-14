use util::{async_trait, error_handling::AppResult};

use crate::output::fabric_switch_output::FabricSwitchOutput;

#[async_trait]
pub trait FabricSwitchService {
    async fn fabric_switches(&self) -> AppResult<Vec<FabricSwitchOutput>>;
}
