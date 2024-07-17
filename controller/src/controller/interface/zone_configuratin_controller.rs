use util::{async_trait, error_handling::AppResult};

use crate::response::{
    effective_configuration_response::EffectiveConfigurationWrapResponse,
    rest_response::RestResponse,
};

#[async_trait]
pub trait ZoneConfigurationController {
    async fn effective_configuration(
        &self,
    ) -> AppResult<RestResponse<EffectiveConfigurationWrapResponse>>;
}
