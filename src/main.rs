#[tokio::main]
async fn main() {
    infra::migration::migrate().await;
    api::start().await
}
