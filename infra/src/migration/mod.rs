use sea_orm::Database;
pub use sea_orm_migration::prelude::*;

use crate::DATABASE_URL;

mod m20220101_000002_create_zone_table;
mod m20220101_000003_create_wwn_table;
mod m20220101_000004_create_zone_configuration_table;
mod m20220101_000005_create_connected_server_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            // Box::new(m20220101_000003_create_wwn_table::Migration),
            Box::new(m20220101_000002_create_zone_table::Migration),
            Box::new(m20220101_000004_create_zone_configuration_table::Migration),
            Box::new(m20220101_000005_create_connected_server_table::Migration),
        ]
    }
}

pub async fn migrate() {
    let db = Database::connect(DATABASE_URL).await.expect("");
    Migrator::up(&db, None).await;
}
