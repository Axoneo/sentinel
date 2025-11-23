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
                    .table(Resources::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Resources::InternalId).big_integer().not_null().primary_key())
                    .col(ColumnDef::new(Resources::NamespaceId).big_integer().not_null())
                    .col(ColumnDef::new(Resources::ExternalId).string().not_null())
                    .col(ColumnDef::new(Resources::ResourceName).string().not_null())
                    .col(ColumnDef::new(Resources::ResourceType).string().not_null())
                    .col(ColumnDef::new(Resources::ParentInternalId).big_integer())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-namespace-resource")
                            .from(Resources::Table, Resources::NamespaceId)
                            .to(Namespaces::Table, Namespaces::Id),
                    )
                    .to_owned(),
            )
            .await?;
        
        manager
            .create_table(
                Table::create()
                    .table(Links::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Links::LinkInternalId).big_integer().not_null().primary_key())
                    .col(ColumnDef::new(Links::TargetInternalId).big_integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-link-resource")
                            .from(Links::Table, Links::LinkInternalId)
                            .to(Resources::Table, Resources::InternalId),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-target-resource")
                            .from(Links::Table, Links::TargetInternalId)
                            .to(Resources::Table, Resources::InternalId),
                    )
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Links::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(Resources::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Resources {
    Table,
    InternalId,
    NamespaceId,
    ExternalId,
    ResourceName,
    ResourceType,
    ParentInternalId,
}

#[derive(DeriveIden)]
pub enum Links {
    Table,
    LinkInternalId,
    TargetInternalId,
}