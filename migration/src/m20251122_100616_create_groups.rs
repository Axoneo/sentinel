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
                    .table(Groups::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(
                        Groups::InternalId
                    ).big_integer().not_null().primary_key())
                    .col(ColumnDef::new(Groups::NamespaceId).string().not_null())
                    .col(ColumnDef::new(Groups::ExternalId).string().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-namespace-group")
                            .from(Groups::Table, Groups::NamespaceId)
                            .to(Namespaces::Table, Namespaces::Id),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx-group-namespace-id-external-id")
                    .table(Groups::Table)
                    .col(Groups::NamespaceId)
                    .col(Groups::ExternalId)
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
                    .name("idx-group-namespace-id-external-id")
                    .table(Groups::Table)
                    .to_owned(),
            )
            .await?;

        manager
            .drop_table(Table::drop().table(Groups::Table).to_owned())
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
pub enum Groups {
    Table,
    InternalId,
    NamespaceId,
    ExternalId,
}