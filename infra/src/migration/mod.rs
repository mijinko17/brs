pub use sea_orm_migration::prelude::*;
use util::{async_trait, error_handling::AppResult, new};

use crate::dao::database_connection_factory::DatabaseConnectionFactory;

mod m20220101_000002_create_zone_table;
mod m20220101_000004_create_zone_configuration_table;
mod m20220101_000005_create_connected_server_table;

#[async_trait]
pub trait Migrator: MigratorTrait {
    async fn migrate(&self) -> AppResult<()>;
}

#[derive(new)]
pub struct MigratorImpl<T: DatabaseConnectionFactory> {
    connection_factory: T,
}

#[async_trait]
impl<T: DatabaseConnectionFactory + Sync + Send> MigratorTrait for MigratorImpl<T> {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000002_create_zone_table::Migration),
            Box::new(m20220101_000004_create_zone_configuration_table::Migration),
            Box::new(m20220101_000005_create_connected_server_table::Migration),
        ]
    }
}

#[async_trait]
impl<T: DatabaseConnectionFactory + Sync + Send> Migrator for MigratorImpl<T> {
    async fn migrate(&self) -> AppResult<()> {
        let db = self.connection_factory.connection().await?;
        Self::up(&db, None).await?;
        Ok(())
    }
}
