use sea_orm::{Database, DatabaseConnection};
use util::{async_trait, error_handling::AppResult};

pub(crate) const DATABASE_URL: &str = "sqlite:./data/database.db?mode=rwc";

#[async_trait]
pub trait DatabaseConnectionFactory {
    async fn connection(&self) -> AppResult<DatabaseConnection>;
}

pub struct DatabaseConnectionFactoryImpl;

#[async_trait]
impl DatabaseConnectionFactory for DatabaseConnectionFactoryImpl {
    async fn connection(&self) -> AppResult<DatabaseConnection> {
        Ok(Database::connect(DATABASE_URL).await?)
    }
}
