use infra::{
    config::{ConfigReader, Wwn, Zone},
    dao::{
        connected_server_dao::ConnectedServerDao,
        zone_dao::{ZoneDao, ZoneEntry},
    },
    entity::connected_server,
};
use injection::{config_reader, connected_server_dao, zone_dao};
use sea_orm::{ActiveValue::NotSet, Set};
use util::{async_trait, error_handling::AppResult, new};

pub async fn import() -> AppResult<()> {
    ImporterImpl::new(config_reader(), connected_server_dao(), zone_dao())
        .import()
        .await
}

#[async_trait]
pub trait Importer {
    async fn import(&self) -> AppResult<()>;
}

#[derive(new)]
pub struct ImporterImpl<T, U, V>
where
    T: ConfigReader,
    U: ConnectedServerDao,
    V: ZoneDao,
{
    config_reader: T,
    connected_server_dao: U,
    zone_dao: V,
}

impl<T, U, V> ImporterImpl<T, U, V>
where
    T: ConfigReader,
    U: ConnectedServerDao,
    V: ZoneDao,
{
    async fn import_connected_server(&self, connected_servers: Vec<Wwn>) -> AppResult<()> {
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

    async fn import_zones(&self, zones: Vec<Zone>) -> AppResult<()> {
        let zone_entries = zones
            .into_iter()
            .map(|zone| {
                ZoneEntry::new(
                    zone.name,
                    zone.members
                        .into_iter()
                        .map(|member| member.value)
                        .collect(),
                )
            })
            .collect();
        self.zone_dao.save(zone_entries).await
    }
}

#[async_trait]
impl<T, U, V> Importer for ImporterImpl<T, U, V>
where
    T: ConfigReader + Sync,
    U: ConnectedServerDao + Sync,
    V: ZoneDao + Sync,
{
    async fn import(&self) -> AppResult<()> {
        let config = self.config_reader.read()?;
        println!("{:?}", config);
        self.import_connected_server(config.initial_setting.connected_server)
            .await?;
        self.import_zones(config.initial_setting.zones).await?;
        Ok(())
    }
}
