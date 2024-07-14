use crate::entity::prelude::Wwn;
use sea_orm::{ActiveModelTrait, ActiveValue, EntityTrait};
use sea_orm::{Database, DbErr};
use usecase::entity::zone::Zone;
use usecase::repository::zone_repository::ZoneRepository;
use util::async_trait;

use crate::DATABASE_URL;

pub struct ZoneRepositoryImpl;

#[async_trait]
impl ZoneRepository for ZoneRepositoryImpl {
    async fn save(&self, zones: Vec<usecase::entity::zone::Zone>) {
        let db = Database::connect(DATABASE_URL).await.expect("");
        let hoge = crate::entity::zone::ActiveModel {
            name: ActiveValue::set(zones.get(0).unwrap().name()),
            ..Default::default()
        };
        println!("{:?}", hoge.insert(&db).await);
    }

    async fn zones(&self) -> Vec<usecase::entity::zone::Zone> {
        let db = Database::connect(DATABASE_URL).await.expect("msg");
        let c = crate::entity::zone::Entity::find()
            .find_with_related(Wwn)
            .all(&db)
            .await
            .expect("msg");
        c.into_iter()
            .map(|(zone, wwns)| {
                Zone::new(
                    zone.name,
                    wwns.into_iter()
                        .map(|wwn| {
                            usecase::entity::wwn::Wwn::new([
                                wwn.v0, wwn.v1, wwn.v2, wwn.v3, wwn.v4, wwn.v5, wwn.v6, wwn.v7,
                            ])
                        })
                        .collect(),
                )
            })
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

fn wwn_from_string(str: String) -> Wwn {
    todo!()
}
