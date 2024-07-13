use sea_orm::{ActiveModelTrait, ActiveValue, EntityTrait};
use sea_orm::{Database, DbErr};
use usecase::entity::zone::Zone;
use usecase::repository::zone_repository::ZoneRepository;
use util::async_trait;

const DATABASE_URL: &str = "sqlite:./piyo.db?mode=rwc";

pub struct ZoneRepositoryImpl;

#[async_trait]
impl ZoneRepository for ZoneRepositoryImpl {
    async fn save(&self, zones: Vec<usecase::entity::zone::Zone>) {
        let db = Database::connect(DATABASE_URL).await.expect("");
        let hoge = super::entity::zone::ActiveModel {
            name: ActiveValue::set(zones.get(0).unwrap().name()),
            ..Default::default()
        };
        println!("{:?}", hoge.insert(&db).await);
    }

    async fn zones(&self) -> Vec<usecase::entity::zone::Zone> {
        let db = Database::connect(DATABASE_URL).await.expect("msg");
        let c = super::entity::zone::Entity::find()
            .all(&db)
            .await
            .expect("msg");
        c.into_iter()
            .map(|model| Zone::new(model.name, vec![]))
            .collect()
    }
    // pub async fn zones(&self) -> Result<Vec<super::zone::Model>, DbErr> {
    //     let db = Database::connect(DATABASE_URL).await?;
    //     let c = super::zone::Entity::find().all(&db).await;
    //     c
    // }

    // pub async fn add_zone(&self, name: &str) -> Result<(), DbErr> {
    //     let db = Database::connect(DATABASE_URL).await?;
    //     let hoge = super::zone::ActiveModel {
    //         name: ActiveValue::set(name.to_string()),
    //         ..Default::default()
    //     };
    //     println!("{:?}", hoge.insert(&db).await);
    //     Ok(())
    // }
}
