use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Zone::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Zone::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Zone::Name).string().not_null())
                    .to_owned(),
            )
            .await?;
        manager
            .create_table(
                Table::create()
                    .table(Wwn::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Wwn::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Wwn::V0).tiny_unsigned().not_null())
                    .col(ColumnDef::new(Wwn::V1).tiny_unsigned().not_null())
                    .col(ColumnDef::new(Wwn::V2).tiny_unsigned().not_null())
                    .col(ColumnDef::new(Wwn::V3).tiny_unsigned().not_null())
                    .col(ColumnDef::new(Wwn::V4).tiny_unsigned().not_null())
                    .col(ColumnDef::new(Wwn::V5).tiny_unsigned().not_null())
                    .col(ColumnDef::new(Wwn::V6).tiny_unsigned().not_null())
                    .col(ColumnDef::new(Wwn::V7).tiny_unsigned().not_null())
                    .col(ColumnDef::new(Wwn::ZoneId).integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("hoge")
                            .to(Zone::Table, Zone::Id)
                            .from(Wwn::Table, Wwn::ZoneId)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Zone::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Zone {
    Table,
    Id,
    Name,
}

#[derive(DeriveIden)]
enum Wwn {
    Table,
    Id,
    V0,
    V1,
    V2,
    V3,
    V4,
    V5,
    V6,
    V7,
    ZoneId,
}
