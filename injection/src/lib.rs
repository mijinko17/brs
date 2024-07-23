use controller::controller::{
    implement::{
        fabric_switch_controller_impl::FabricSwitchControllerImpl,
        fibrechannel_name_server_controller_impl::FibrechannelNameServerControllerImpl,
        zone_configuration_controller_impl::ZoneConfigurationControllerImpl,
        zone_controller_impl::ZoneControllerImpl,
    },
    interface::{
        fabric_switch_controller::FabricSwitchController,
        fibrechannel_name_server_controller::FibrechannelNameServerController,
        zone_configuratin_controller::ZoneConfigurationController, zone_controller::ZoneController,
    },
};
use domain::{
    repository::{
        connected_server_repository::ConnectedServerRepository,
        fabric_switch_repository::FabricSwitchRespistory,
        zone_configuration_repository::ZoneConfigurationRepository,
        zone_repository::ZoneRepository,
    },
    service::{
        implement::{
            connected_server_service_impl::ConnectedServerServiceImpl,
            fabric_switch_service_impl::FabricSwitchServiceImpl,
            zone_configuration_service_impl::ZoneConfigurationServiceImpl,
            zone_service_impl::ZoneServiceImpl,
        },
        interface::{
            connected_server_service::ConnectedServerService,
            fabric_switch_service::FabricSwitchService,
            zone_configuration_service::ZoneConfigurationService, zone_service::ZoneService,
        },
    },
};
use importer::ImporterImpl;
use infra::{
    config::{ConfigReader, ConfigReaderImpl},
    dao::{
        connected_server_dao::{ConnectedServerDao, ConnectedServerDaoImpl},
        database_connection_factory::{DatabaseConnectionFactory, DatabaseConnectionFactoryImpl},
        zone_configuration_dao::{ZoneConfigurationDao, ZoneConfigurationDaoImpl},
        zone_dao::{ZoneDao, ZoneDaoImpl},
    },
    migration::MigratorImpl,
    repository::{
        connected_server_repository_impl::ConnectedServerRepositoryImpl,
        fabric_switch_repository_impl::FabricSwitchRespistoryImpl,
        zone_configuration_repository_impl::ZoneConfigurationRepositoryImpl,
        zone_repository_impl::ZoneRepositoryImpl,
    },
};

pub use importer::Importer;
pub use infra::migration::Migrator;

pub fn database_connection_factory() -> impl DatabaseConnectionFactory {
    DatabaseConnectionFactoryImpl
}

pub fn zone_dao() -> impl ZoneDao {
    ZoneDaoImpl::new(database_connection_factory())
}

pub fn zone_configuration_dao() -> impl ZoneConfigurationDao {
    ZoneConfigurationDaoImpl::new(database_connection_factory())
}

pub fn connected_server_dao() -> impl ConnectedServerDao {
    ConnectedServerDaoImpl::new(database_connection_factory())
}

pub fn zone_repository() -> impl ZoneRepository {
    ZoneRepositoryImpl::new(zone_dao())
}

pub fn zone_configuration_repository() -> impl ZoneConfigurationRepository {
    ZoneConfigurationRepositoryImpl::new(zone_configuration_dao())
}

pub fn fabric_switch_repository() -> impl FabricSwitchRespistory {
    FabricSwitchRespistoryImpl
}

pub fn connected_server_repository() -> impl ConnectedServerRepository {
    ConnectedServerRepositoryImpl::new(connected_server_dao())
}

pub fn zone_service() -> impl ZoneService {
    ZoneServiceImpl::new(zone_repository())
}

pub fn zone_configuration_service() -> impl ZoneConfigurationService {
    ZoneConfigurationServiceImpl::new(zone_configuration_repository())
}

pub fn fabric_switch_service() -> impl FabricSwitchService {
    FabricSwitchServiceImpl::new(fabric_switch_repository())
}

pub fn connected_server_service() -> impl ConnectedServerService {
    ConnectedServerServiceImpl::new(connected_server_repository())
}

pub fn zone_configuratin_controller() -> impl ZoneConfigurationController {
    ZoneConfigurationControllerImpl::new(zone_configuration_service())
}

pub fn fabric_switch_controller() -> impl FabricSwitchController {
    FabricSwitchControllerImpl::new(fabric_switch_service())
}

pub fn zone_controller() -> impl ZoneController {
    ZoneControllerImpl::new(zone_service())
}

pub fn fibrechannel_name_server_controller() -> impl FibrechannelNameServerController {
    FibrechannelNameServerControllerImpl::new(connected_server_service())
}

pub fn config_reader() -> impl ConfigReader {
    ConfigReaderImpl
}

pub fn migrator() -> impl Migrator {
    MigratorImpl::new(database_connection_factory())
}

pub fn importer() -> impl Importer {
    ImporterImpl::new(config_reader(), connected_server_dao(), zone_dao())
}
