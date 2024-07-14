use usecase::service::interface::connected_server_service::ConnectedServerService;
use util::{async_trait, error_handling::AppResult, new};

use crate::{
    controller::interface::fibrechannel_name_server_controller::FibrechannelNameServerController,
    response::{
        connected_server_response::{
            FibrechannelNameServerResponse, FibrechannelNameServerWrapResponse,
        },
        rest_response::RestResponse,
    },
    util::wwn::format_wwn,
};

#[derive(new)]
pub struct FibrechannelNameServerControllerImpl<T: ConnectedServerService> {
    connected_server_service: T,
}

#[async_trait]
impl<T: ConnectedServerService + Sync> FibrechannelNameServerController
    for FibrechannelNameServerControllerImpl<T>
{
    async fn fibrechannel_name_server(
        &self,
    ) -> AppResult<RestResponse<FibrechannelNameServerWrapResponse>> {
        let response = FibrechannelNameServerWrapResponse::new(
            self.connected_server_service
                .connected_servers()
                .await?
                .into_iter()
                .map(|connected_server_output| {
                    FibrechannelNameServerResponse::new(format_wwn(connected_server_output.wwn))
                })
                .collect(),
        );
        Ok(RestResponse::new(response))
    }
}
