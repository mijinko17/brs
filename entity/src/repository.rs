use sea_orm::{ActiveModelTrait, ActiveValue, EntityTrait};
use sea_orm::{Database, DbErr};

const DATABASE_URL: &str = "sqlite:./piyo.db?mode=rwc";

pub struct ZoneRepositoryImpl;

impl ZoneRepositoryImpl {
    pub async fn zones(&self) -> Result<Vec<super::zone::Model>, DbErr> {
        let db = Database::connect(DATABASE_URL).await?;
        let c = super::zone::Entity::find().all(&db).await;
        c
    }

    pub async fn add_zone(&self, name: &str) -> Result<(), DbErr> {
        let db = Database::connect(DATABASE_URL).await?;
        let hoge = super::zone::ActiveModel {
            name: ActiveValue::set(name.to_string()),
            ..Default::default()
        };
        println!("{:?}", hoge.insert(&db).await);
        Ok(())
    }
}
