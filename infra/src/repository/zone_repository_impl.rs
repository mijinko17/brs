use crate::dao::zone_dao::{ZoneDao, ZoneEntry};
use crate::entity::prelude::Wwn;
use crate::DATABASE_URL;
use sea_orm::Database;
use sea_orm::{ActiveModelTrait, ActiveValue, EntityTrait};
use usecase::entity::zone::Zone;
use usecase::repository::zone_repository::ZoneRepository;
use util::error_handling::AppResult;
use util::{async_trait, new};

#[derive(new)]
pub struct ZoneRepositoryImpl<T>
where
    T: ZoneDao,
{
    zone_dao: T,
}

#[async_trait]
impl<T> ZoneRepository for ZoneRepositoryImpl<T>
where
    T: ZoneDao + Sync,
{
    async fn save(&self, zones: Vec<usecase::entity::zone::Zone>) -> AppResult<()> {
        let zone_entries = zones
            .into_iter()
            .map(|zone| {
                ZoneEntry::new(
                    zone.name(),
                    zone.members()
                        .into_iter()
                        .map(|member| member.value())
                        .collect(),
                )
            })
            .collect();
        self.zone_dao.save(zone_entries).await
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
