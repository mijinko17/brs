use util::{async_trait, error_handling::AppResult};

use crate::response::{
    connected_server_response::FibrechannelNameServerWrapResponse, rest_response::RestResponse,
};

#[async_trait]
pub trait FibrechannelNameServerController {
    async fn fibrechannel_name_server(
        &self,
    ) -> AppResult<RestResponse<FibrechannelNameServerWrapResponse>>;
}
