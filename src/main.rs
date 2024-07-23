use injection::{importer, migrator, Importer, Migrator};
use util::error_handling::AppResult;

#[tokio::main]
async fn main() -> AppResult<()> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .with_test_writer()
        .init();

    migrator().migrate().await?;
    importer().import().await?;
    api::start().await;
    Ok(())
}
