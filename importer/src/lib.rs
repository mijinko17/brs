use infra::{
    config::{ConfigReader, Wwn, ZoneConfigurationConfig},
    dao::{
        connected_server_dao::ConnectedServerDao,
        wwn_dao::WwnDao,
        zone_configuration_dao::ZoneConfigurationDao,
        zone_dao::ZoneDao,
    },
    entity::{
        connected_server, wwn, zone,
        zone_configuration::{self},
    },
};
use sea_orm::{ActiveValue::NotSet, Set};
use util::{async_trait, error_handling::AppResult, new};

#[async_trait]
pub trait Importer {
    async fn import(&self) -> AppResult<()>;
}

#[derive(new)]
pub struct ImporterImpl<T, U, V, W, X>
where
    T: ConfigReader,
    U: ConnectedServerDao,
    V: ZoneDao,
    W: ZoneConfigurationDao,
    X: WwnDao,
{
    config_reader: T,
    connected_server_dao: U,
    zone_dao: V,
    zone_configuration_dao: W,
    wwn_dao: X,
}

impl<T, U, V, W, X> ImporterImpl<T, U, V, W, X>
where
    T: ConfigReader,
    U: ConnectedServerDao,
    V: ZoneDao,
    W: ZoneConfigurationDao,
    X: WwnDao,
{
    async fn import_connected_server(&self, connected_servers: Vec<Wwn>) -> AppResult<()> {
        self.connected_server_dao.delete_all().await?;
        let connedted_servers_active_model = connected_servers
            .into_iter()
            .map(|wwn| connected_server::ActiveModel {
                id: NotSet,
                v0: Set(wwn.value[0]),
                v1: Set(wwn.value[1]),
                v2: Set(wwn.value[2]),
                v3: Set(wwn.value[3]),
                v4: Set(wwn.value[4]),
                v5: Set(wwn.value[5]),
                v6: Set(wwn.value[6]),
                v7: Set(wwn.value[7]),
            })
            .collect();
        self.connected_server_dao
            .save(connedted_servers_active_model)
            .await
    }

    async fn import_zone_configurations(
        &self,
        zone_confiugrations: Vec<ZoneConfigurationConfig>,
    ) -> AppResult<()> {
        self.zone_dao.delete_all().await?;
        for zone_configuration in zone_confiugrations {
            let result_zone_configuration = self
                .zone_configuration_dao
                .save(zone_configuration::ActiveModel {
                    id: NotSet,
                    name: Set(zone_configuration.name),
                    is_effective: Set(zone_configuration.is_effective),
                })
                .await?;
            for zone in zone_configuration.zones {
                let result_zone = self
                    .zone_dao
                    .save(zone::ActiveModel {
                        id: NotSet,
                        name: Set(zone.name),
                        cfg_id: Set(Some(result_zone_configuration.id)),
                    })
                    .await?;
                self.wwn_dao
                    .save(
                        zone.members
                            .into_iter()
                            .map(|wwn| wwn.value)
                            .map(|[v0, v1, v2, v3, v4, v5, v6, v7]| wwn::ActiveModel {
                                id: NotSet,
                                v0: Set(v0),
                                v1: Set(v1),
                                v2: Set(v2),
                                v3: Set(v3),
                                v4: Set(v4),
                                v5: Set(v5),
                                v6: Set(v6),
                                v7: Set(v7),
                                zone_id: Set(result_zone.id),
                            })
                            .collect(),
                    )
                    .await?;
            }
        }
        Ok(())
    }
}

#[async_trait]
impl<T, U, V, W, X> Importer for ImporterImpl<T, U, V, W, X>
where
    T: ConfigReader + Sync,
    U: ConnectedServerDao + Sync,
    V: ZoneDao + Sync,
    W: ZoneConfigurationDao + Sync,
    X: WwnDao + Sync,
{
    async fn import(&self) -> AppResult<()> {
        let config = self.config_reader.read()?;
        self.import_connected_server(config.initial_setting.connected_server)
            .await?;
        self.import_zone_configurations(config.initial_setting.zone_configurations)
            .await?;
        Ok(())
    }
}
