use util::{async_trait, error_handling::AppResult};

use crate::entity::{
    zone::ZoneIdentifier,
    zone_configuration::{ZoneConfiguration, ZoneConfigurationIdentifier},
};

#[async_trait]
pub trait ZoneConfigurationRepository {
    async fn effective_configuration(
        &self,
    ) -> AppResult<Option<(ZoneConfigurationIdentifier, ZoneConfiguration)>>;
    async fn zone_configuration_by_name(
        &self,
        name: String,
    ) -> AppResult<Option<(ZoneConfigurationIdentifier, ZoneConfiguration)>>;
    async fn update_zone_member(
        &self,
        zone_configuration_identifier: ZoneConfigurationIdentifier,
        zone_member: Vec<ZoneIdentifier>,
    ) -> AppResult<()>;
    async fn create(&self, name: String, zone_member: Vec<ZoneIdentifier>) -> AppResult<()>;
    async fn enable_zone_configuration(
        &self,
        zone_configuration_identifier: ZoneConfigurationIdentifier,
    ) -> AppResult<()>;
}
