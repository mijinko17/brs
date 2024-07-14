use controller::controller::{
    implement::{
        fabric_switch_controller_impl::FabricSwitchControllerImpl,
        zone_configuration_controller_impl::ZoneConfigurationControllerImpl,
    },
    interface::{
        fabric_switch_controller::FabricSwitchController,
        zone_configuratin_controller::ZoneConfigurationController,
    },
};
use infra::repository::{
    fabric_switch_repository_impl::FabricSwitchRespistoryImpl,
    zone_repository_impl::ZoneRepositoryImpl,
};
use usecase::{
    repository::{
        fabric_switch_repository::FabricSwitchRespistory, zone_repository::ZoneRepository,
    },
    service::{
        implement::{
            fabric_switch_service_impl::FabricSwitchServiceImpl, zone_service_impl::ZoneServiceImpl,
        },
        interface::{fabric_switch_service::FabricSwitchService, zone_service::ZoneService},
    },
};

pub fn zone_repository() -> impl ZoneRepository {
    ZoneRepositoryImpl
}

pub fn fabric_switch_repository() -> impl FabricSwitchRespistory {
    FabricSwitchRespistoryImpl
}

pub fn zone_service() -> impl ZoneService {
    ZoneServiceImpl::new(zone_repository())
}

pub fn fabric_switch_service() -> impl FabricSwitchService {
    FabricSwitchServiceImpl::new(fabric_switch_repository())
}

pub fn zone_configuratin_controller() -> impl ZoneConfigurationController {
    ZoneConfigurationControllerImpl::new(zone_service())
}

pub fn fabric_switch_controller() -> impl FabricSwitchController {
    FabricSwitchControllerImpl::new(fabric_switch_service())
}

#[cfg(test)]
mod tests {
    use super::*;
}
