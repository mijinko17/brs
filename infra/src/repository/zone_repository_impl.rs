use crate::dao::zone_dao::{DeleteZoneEntry, ZoneDao, ZoneEntry};
use domain::entity::zone::Zone;
use domain::repository::zone_repository::ZoneRepository;
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
    async fn save(&self, zones: Vec<domain::entity::zone::Zone>) -> AppResult<()> {
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

    async fn delete_by_name(&self, delete_zone_name: Vec<String>) -> AppResult<()> {
        self.zone_dao
            .delete(delete_zone_name.into_iter().map(DeleteZoneEntry).collect())
            .await
    }

    async fn zones(&self) -> AppResult<Vec<domain::entity::zone::Zone>> {
        let models = self.zone_dao.zones().await?;
        Ok(models
            .into_iter()
            .map(|(zone, wwns)| {
                Zone::new(
                    zone.name,
                    wwns.into_iter()
                        .map(|wwn| {
                            domain::entity::wwn::Wwn::new([
                                wwn.v0, wwn.v1, wwn.v2, wwn.v3, wwn.v4, wwn.v5, wwn.v6, wwn.v7,
                            ])
                        })
                        .collect(),
                )
            })
            .collect())
    }
}
