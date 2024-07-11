use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        // todo!();

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
                    .col(ColumnDef::new(Wwn::Value).string().not_null())
                    .col(ColumnDef::new(Wwn::ZoneId).integer().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        // todo!();

        manager
            .drop_table(Table::drop().table(Wwn::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Wwn {
    Table,
    Id,
    Value,
    ZoneId,
}
