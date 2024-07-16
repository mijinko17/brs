use util::error_handling::AppResult;

#[tokio::main]
async fn main() -> AppResult<()> {
    // print!("{:?}", ConfigReaderImpl.read());
    infra::migration::migrate().await;
    importer::import().await?;
    Ok(())
    // infra::migration::migrate().await;
    // api::start().await
}
