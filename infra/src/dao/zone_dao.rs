use sea_orm::{
    ActiveValue::NotSet, ColumnTrait, Database, EntityTrait, ModelTrait, QueryFilter, Set,
};
use util::{async_trait, error_handling::AppResult, new, Context};

use crate::{
    entity::{wwn, zone},
    DATABASE_URL,
};

#[derive(Clone, new)]
pub struct ZoneEntry {
    name: String,
    members: Vec<[u8; 8]>,
}

pub struct DeleteZoneEntry(pub String);

#[async_trait]
pub trait ZoneDao {
    async fn save(&self, zones: Vec<ZoneEntry>) -> AppResult<()>;
    async fn delete(&self, delete_zones: Vec<DeleteZoneEntry>) -> AppResult<()>;
}

pub struct ZoneDaoImpl;

#[async_trait]
impl ZoneDao for ZoneDaoImpl {
    async fn save(&self, zones: Vec<ZoneEntry>) -> AppResult<()> {
        let db = Database::connect(DATABASE_URL).await?;
        for zone_entry in zones.clone() {
            let z = zone::ActiveModel {
                id: NotSet,
                name: Set(zone_entry.name),
            };
            let a = zone::Entity::insert(z).exec_with_returning(&db).await?;
            let m = zone_entry
                .members
                .into_iter()
                .map(|wwn| wwn::ActiveModel {
                    id: NotSet,
                    v0: Set(wwn[0]),
                    v1: Set(wwn[1]),
                    v2: Set(wwn[2]),
                    v3: Set(wwn[3]),
                    v4: Set(wwn[4]),
                    v5: Set(wwn[5]),
                    v6: Set(wwn[6]),
                    v7: Set(wwn[7]),
                    zone_id: Set(a.id),
                })
                .collect::<Vec<_>>();
            wwn::Entity::insert_many(m)
                .on_empty_do_nothing()
                .exec(&db)
                .await?;
        }
        Ok(())
    }

    async fn delete(&self, delete_zones: Vec<DeleteZoneEntry>) -> AppResult<()> {
        let db = Database::connect(DATABASE_URL).await?;
        let mut delete_targets = vec![];
        for delete_zone in delete_zones {
            let target = zone::Entity::find()
                .filter(zone::Column::Name.eq(delete_zone.0.clone()))
                .one(&db)
                .await?
                .context(format!("Not found zone {}", delete_zone.0))?;
            delete_targets.push(target);
        }
        for target in delete_targets {
            target.delete(&db).await?;
        }
        Ok(())
    }
}
