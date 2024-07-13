// use infra::repository::ZoneRepositoryImpl;
use sea_orm::{ActiveModelTrait, ActiveValue, Database, DbErr, EntityTrait, InsertResult};
// use usecase::{
//     input::{create_zone_input::CreateZoneInput, create_zones_input::CreateZonesInput},
//     service::{
//         implement::zone_service_impl::ZoneServiceImpl, interface::zone_service::ZoneService,
//     },
// };

// Change this according to your database implementation,
// or supply it as an environment variable.
// the whole database URL string follows the following format:
// "protocol://username:password@host:port/database"
// We put the database name (that last bit) in a separate variable simply for convenience.
const DATABASE_URL: &str = "sqlite:./piyo.db?mode=rwc";
const DB_NAME: &str = "bakeries_db";

async fn run() -> Result<(), DbErr> {
    let db = Database::connect(DATABASE_URL).await?;
    println!("aaa");
    let hoge = entity::post::ActiveModel {
        text: ActiveValue::set("textxt".to_owned()),
        title: ActiveValue::set("titile".to_owned()),
        ..Default::default()
    };
    println!("{:?}", hoge.insert(&db).await);
    Ok(())
}

#[tokio::main]
async fn main() {
    // if let Err(err) = block_on(run()) {
    // panic!("{}", err);
    // }
    api::start().await
    // println!("{:?}", ZoneRepositoryImpl.add_zone("hogehoge").await);
    // println!("{:?}", ZoneRepositoryImpl.add_zone("fugafuga").await);
    // println!("{:?}", ZoneRepositoryImpl.zones().await);
    // let service = ZoneServiceImpl::new(ZoneRepositoryImpl);
    // println!(
    //     "{:?}",
    //     service
    //         .create_zones(CreateZonesInput::new(vec![CreateZoneInput::new(
    //             "hoge".to_string(),
    //             vec![]
    //         )]))
    //         .await
    // );
    // println!("{:?}", service.zones().await);
}
