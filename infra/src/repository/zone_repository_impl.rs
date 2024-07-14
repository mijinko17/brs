use crate::entity::prelude::Wwn;
use crate::DATABASE_URL;
use sea_orm::Database;
use sea_orm::{ActiveModelTrait, ActiveValue, EntityTrait};
use usecase::entity::zone::Zone;
use usecase::repository::zone_repository::ZoneRepository;
use util::async_trait;
use util::error_handling::AppResult;

pub struct ZoneRepositoryImpl;

#[async_trait]
impl ZoneRepository for ZoneRepositoryImpl {
    async fn save(&self, zones: Vec<usecase::entity::zone::Zone>) -> AppResult<()> {
        let db = Database::connect(DATABASE_URL).await?;
        let model = crate::entity::zone::ActiveModel {
            name: ActiveValue::set(zones.get(0).unwrap().name()),
            ..Default::default()
        };
        model.insert(&db).await?;
        Ok(())
    }

    async fn zones(&self) -> AppResult<Vec<usecase::entity::zone::Zone>> {
        let db = Database::connect(DATABASE_URL).await?;
        let models = crate::entity::zone::Entity::find()
            .find_with_related(Wwn)
            .all(&db)
            .await?;
        Ok(models
            .into_iter()
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
            .collect())
    }
}
