use controller::controller::{
    implement::{
        fabric_switch_controller_impl::FabricSwitchControllerImpl,
        fibrechannel_name_server_controller_impl::FibrechannelNameServerControllerImpl,
        zone_configuration_controller_impl::ZoneConfigurationControllerImpl,
    },
    interface::{
        fabric_switch_controller::FabricSwitchController,
        fibrechannel_name_server_controller::FibrechannelNameServerController,
        zone_configuratin_controller::ZoneConfigurationController,
    },
};
use infra::{
    config::{ConfigReader, ConfigReaderImpl},
    dao::connected_server_dao::{ConnectedServerDao, ConnectedServerDaoImpl},
    import::{Importer, ImporterImpl},
    repository::{
        connected_server_repository_impl::ConnectedServerRepositoryImpl,
        fabric_switch_repository_impl::FabricSwitchRespistoryImpl,
        zone_repository_impl::ZoneRepositoryImpl,
    },
};
use usecase::{
    repository::{
        connected_server_repository::ConnectedServerRepository,
        fabric_switch_repository::FabricSwitchRespistory, zone_repository::ZoneRepository,
    },
    service::{
        implement::{
            connected_server_service_impl::ConnectedServerServiceImpl,
            fabric_switch_service_impl::FabricSwitchServiceImpl,
            zone_service_impl::ZoneServiceImpl,
        },
        interface::{
            connected_server_service::ConnectedServerService,
            fabric_switch_service::FabricSwitchService, zone_service::ZoneService,
        },
    },
};

pub fn zone_repository() -> impl ZoneRepository {
    ZoneRepositoryImpl
}

pub fn fabric_switch_repository() -> impl FabricSwitchRespistory {
    FabricSwitchRespistoryImpl
}

pub fn connected_server_repository() -> impl ConnectedServerRepository {
    ConnectedServerRepositoryImpl
}

pub fn zone_service() -> impl ZoneService {
    ZoneServiceImpl::new(zone_repository())
}

pub fn fabric_switch_service() -> impl FabricSwitchService {
    FabricSwitchServiceImpl::new(fabric_switch_repository())
}

pub fn connected_server_service() -> impl ConnectedServerService {
    ConnectedServerServiceImpl::new(connected_server_repository())
}

pub fn zone_configuratin_controller() -> impl ZoneConfigurationController {
    ZoneConfigurationControllerImpl::new(zone_service())
}

pub fn fabric_switch_controller() -> impl FabricSwitchController {
    FabricSwitchControllerImpl::new(fabric_switch_service())
}

pub fn fibrechannel_name_server_controller() -> impl FibrechannelNameServerController {
    FibrechannelNameServerControllerImpl::new(connected_server_service())
}

pub fn config_reader() -> impl ConfigReader {
    ConfigReaderImpl
}

pub fn connected_server_dao() -> impl ConnectedServerDao {
    ConnectedServerDaoImpl
}

pub fn importer() -> impl Importer {
    ImporterImpl::new(config_reader(), connected_server_dao())
}

#[cfg(test)]
mod tests {
    use super::*;
}
