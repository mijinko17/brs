use tracing::{event, info, Level};
use util::error_handling::AppResult;

#[tokio::main]
async fn main() -> AppResult<()> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .with_test_writer()
        .init();
    event!(Level::INFO, "Event_1");
    let number_of_yaks = 3;
    // this creates a new event, outside of any spans.
    info!(number_of_yaks, "preparing to shave yaks");

    // print!("{:?}", ConfigReaderImpl.read());
    let _ = infra::migration::migrate().await;
    println!("{:?}", importer::import().await);
    api::start().await;
    Ok(())
    // infra::migration::migrate().await;
    // api::start().await
}
