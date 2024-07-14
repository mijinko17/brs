use util::{async_trait, error_handling::AppResult};

use crate::entity::fabric_switch::FabricSwitch;

#[async_trait]
pub trait FabricSwitchRespistory {
    async fn fabric_switches(&self) -> AppResult<Vec<FabricSwitch>>;
}
