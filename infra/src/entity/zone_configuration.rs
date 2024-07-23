//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.15

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "zone_configuration")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub name: String,
    pub is_effective: bool,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::zone::Entity")]
    Zone,
}

impl Related<super::zone::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Zone.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
