use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Namespaces::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Namespaces::Id).big_integer().not_null().primary_key())
                    .col(ColumnDef::new(Namespaces::Name).string().not_null())
                    .col(ColumnDef::new(Namespaces::Secret).string().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Namespaces::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Namespaces {
    Table,
    Id,
    Name,
    Secret
}
