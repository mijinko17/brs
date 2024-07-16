use sea_orm::{ActiveValue::NotSet, Set};
use util::{async_trait, error_handling::AppResult, new};

use crate::{
    config::ConfigReader, dao::connected_server_dao::ConnectedServerDao, entity::connected_server,
};

#[async_trait]
pub trait Importer {
    async fn import(&self) -> AppResult<()>;
}

#[derive(new)]
pub struct ImporterImpl<T, U>
where
    T: ConfigReader,
    U: ConnectedServerDao,
{
    config_reader: T,
    connected_server_dao: U,
}

#[async_trait]
impl<T, U> Importer for ImporterImpl<T, U>
where
    T: ConfigReader + Sync,
    U: ConnectedServerDao + Sync,
{
    async fn import(&self) -> AppResult<()> {
        let config = self.config_reader.read()?;
        let connedted_servers_active_model = config
            .initial_setting
            .connected_server
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
}
