use controller::controller::{
    implement::zone_configuration_controller_impl::ZoneConfigurationControllerImpl,
    interface::zone_configuratin_controller::ZoneConfigurationController,
};
use infra::repository::ZoneRepositoryImpl;
use usecase::{
    repository::zone_repository::ZoneRepository,
    service::{
        implement::zone_service_impl::ZoneServiceImpl, interface::zone_service::ZoneService,
    },
};

pub fn zone_repository() -> impl ZoneRepository {
    ZoneRepositoryImpl
}

pub fn zone_service() -> impl ZoneService {
    ZoneServiceImpl::new(zone_repository())
}

pub fn zone_configuratin_controller() -> impl ZoneConfigurationController {
    ZoneConfigurationControllerImpl::new(zone_service())
}

#[cfg(test)]
mod tests {
    use super::*;
}
