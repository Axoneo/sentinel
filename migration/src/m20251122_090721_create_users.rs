use sea_orm_migration::{prelude::*, schema::*};

use crate::m20251122_070217_create_namespaces::Namespaces;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Users::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(
                        Users::InternalId
                    ).big_integer().not_null().primary_key())
                    .col(ColumnDef::new(Users::NamespaceId).big_integer().not_null())
                    .col(ColumnDef::new(Users::ExternalId).string().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-namespace-user")
                            .from(Users::Table, Users::NamespaceId)
                            .to(Namespaces::Table, Namespaces::Id),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx-user-namespace-id-external-id")
                    .table(Users::Table)
                    .col(Users::NamespaceId)
                    .col(Users::ExternalId)
                    .unique()
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_index(
                Index::drop()
                    .name("idx-user-namespace-id-external-id")
                    .table(Users::Table)
                    .to_owned(),
            )
            .await?;

        manager
            .drop_table(Table::drop().table(Users::Table).to_owned())
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
pub enum Users {
    Table,
    InternalId,
    NamespaceId,
    ExternalId,
}