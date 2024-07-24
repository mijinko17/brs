use domain::{
    entity::{
        zone::{Zone, ZoneIdentifier},
        zone_configuration::{ZoneConfiguration, ZoneConfigurationIdentifier},
    },
    repository::zone_configuration_repository::ZoneConfigurationRepository,
};
use sea_orm::{ActiveValue::NotSet, ColumnTrait, Set};
use util::{async_trait, error_handling::AppResult, new};

use crate::{
    dao::{zone_configuration_dao::ZoneConfigurationDao, zone_dao::ZoneDao},
    entity::{wwn, zone, zone_configuration},
};

#[derive(new)]
pub struct ZoneConfigurationRepositoryImpl<T: ZoneConfigurationDao, U: ZoneDao> {
    zone_configuration_dao: T,
    zone_dao: U,
}

#[async_trait]
impl<T: ZoneConfigurationDao + Sync, U: ZoneDao + Sync> ZoneConfigurationRepository
    for ZoneConfigurationRepositoryImpl<T, U>
{
    async fn effective_configuration(
        &self,
    ) -> AppResult<Option<(ZoneConfigurationIdentifier, ZoneConfiguration)>> {
        let a = self
            .zone_configuration_dao
            .effective_configuration()
            .await?;
        let b = a.map(convert_model);
        Ok(b)
    }

    async fn zone_configuration_by_name(
        &self,
        name: String,
    ) -> AppResult<Option<(ZoneConfigurationIdentifier, ZoneConfiguration)>> {
        let model = self
            .zone_configuration_dao
            .zone_configuration_by_name(name)
            .await?;
        Ok(model.map(convert_model))
    }

    async fn update_zone_member(
        &self,
        zone_configuration_id: ZoneConfigurationIdentifier,
        zone_member: Vec<ZoneIdentifier>,
    ) -> AppResult<()> {
        let current_members = self
            .zone_dao
            .zones_with_filter(zone::Column::CfgId.eq(zone_configuration_id.clone().0))
            .await?;
        let old_active_models: Vec<zone::ActiveModel> = current_members
            .into_iter()
            .map(|(zone, _)| zone)
            .map(|model| model.into())
            .map(|mut active_model: zone::ActiveModel| {
                active_model.cfg_id = Set(None);
                active_model
            })
            .collect();
        self.zone_dao.update(old_active_models).await?;
        let new_members = self
            .zone_dao
            .zones_with_filter(zone::Column::Id.is_in(zone_member.into_iter().map(|id| id.0)))
            .await?;
        let new_active_models = new_members
            .into_iter()
            .map(|(model, _)| model.into())
            .map(|mut active_model: zone::ActiveModel| {
                active_model.cfg_id = Set(Some(zone_configuration_id.0));
                active_model
            })
            .collect();
        self.zone_dao.update(new_active_models).await?;
        Ok(())
    }

    async fn create(&self, name: String, zone_member: Vec<ZoneIdentifier>) -> AppResult<()> {
        let model = self
            .zone_configuration_dao
            .save(zone_configuration::ActiveModel {
                id: NotSet,
                name: Set(name),
                is_effective: Set(false),
            })
            .await?;
        let new_members = self
            .zone_dao
            .zones_with_filter(zone::Column::Id.is_in(zone_member.into_iter().map(|id| id.0)))
            .await?;
        let new_active_models = new_members
            .into_iter()
            .map(|(model, _)| model.into())
            .map(|mut active_model: zone::ActiveModel| {
                active_model.cfg_id = Set(Some(model.id));
                active_model
            })
            .collect();
        self.zone_dao.update(new_active_models).await?;
        Ok(())
    }
    async fn enable_zone_configuration(
        &self,
        zone_configuration_identifier: ZoneConfigurationIdentifier,
    ) -> AppResult<()> {
        let models = self.zone_configuration_dao.zone_configurations().await?;
        let updates: Vec<zone_configuration::ActiveModel> = models
            .into_iter()
            .map(|model| {
                let id = model.id;
                let mut active_model: zone_configuration::ActiveModel = model.into();
                active_model.is_effective = Set(id == zone_configuration_identifier.0);
                active_model
            })
            .collect();
        self.zone_configuration_dao.update(updates).await?;
        Ok(())
    }
}

fn convert_model(
    (config, zone_with_wwn): (
        zone_configuration::Model,
        Vec<(zone::Model, Vec<wwn::Model>)>,
    ),
) -> (ZoneConfigurationIdentifier, ZoneConfiguration) {
    (
        ZoneConfigurationIdentifier::new(config.id),
        ZoneConfiguration::new(
            config.name,
            zone_with_wwn
                .into_iter()
                .map(|(zone, wwns)| {
                    Zone::new(
                        zone.name,
                        wwns.into_iter()
                            .map(|wwn| {
                                domain::entity::wwn::Wwn::new([
                                    wwn.v0, wwn.v1, wwn.v2, wwn.v3, wwn.v4, wwn.v5, wwn.v6, wwn.v7,
                                ])
                            })
                            .collect(),
                    )
                })
                .collect(),
        ),
    )
}
