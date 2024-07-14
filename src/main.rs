use local_ip_address::local_ip;

// use infra::repository::ZoneRepositoryImpl;
#[tokio::main]
async fn main() {
    let my_local_ip = local_ip().unwrap();
    println!("This is my local IP address: {:?}", my_local_ip);
    infra::migration::migrate().await;
    api::start().await
}
