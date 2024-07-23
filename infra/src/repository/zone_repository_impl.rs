use crate::dao::wwn_dao::WwnDao;
use crate::dao::zone_dao::{DeleteZoneEntry, ZoneDao};
use crate::entity::{wwn, zone};
use domain::entity::zone::Zone;
use domain::repository::zone_repository::ZoneRepository;
use sea_orm::ActiveValue::NotSet;
use sea_orm::Set;
use util::error_handling::AppResult;
use util::{async_trait, new};

#[derive(new)]
pub struct ZoneRepositoryImpl<T, U>
where
    T: ZoneDao,
    U: WwnDao,
{
    zone_dao: T,
    wwn_dao: U,
}

#[async_trait]
impl<T, U> ZoneRepository for ZoneRepositoryImpl<T, U>
where
    T: ZoneDao + Sync,
    U: WwnDao + Sync,
{
    async fn save(&self, zones: Vec<domain::entity::zone::Zone>) -> AppResult<()> {
        for zone in zones {
            let result_zone = self
                .zone_dao
                .save(zone::ActiveModel {
                    id: NotSet,
                    name: Set(zone.name()),
                    cfg_id: NotSet,
                })
                .await?;
            self.wwn_dao
                .save(
                    zone.members()
                        .into_iter()
                        .map(|wwn| wwn.value())
                        .map(|[v0, v1, v2, v3, v4, v5, v6, v7]| wwn::ActiveModel {
                            id: NotSet,
                            v0: Set(v0),
                            v1: Set(v1),
                            v2: Set(v2),
                            v3: Set(v3),
                            v4: Set(v4),
                            v5: Set(v5),
                            v6: Set(v6),
                            v7: Set(v7),
                            zone_id: Set(result_zone.id),
                        })
                        .collect(),
                )
                .await?;
        }
        Ok(())
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
