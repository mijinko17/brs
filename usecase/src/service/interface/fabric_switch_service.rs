use util::async_trait;

use crate::output::fabric_switch_output::FabricSwitchOutput;

#[async_trait]
pub trait FabricSwitchService {
    async fn fabric_switches(&self) -> Vec<FabricSwitchOutput>;
}
