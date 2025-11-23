use sea_orm_migration::{prelude::*, schema::*};

use crate::m20251122_070408_create_resources::Resources;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Permissions::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Permissions::ResourceInternalId).big_integer().not_null())
                    .col(ColumnDef::new(Permissions::SubjectInternalId).big_integer())
                    .col(ColumnDef::new(Permissions::SubjectType).big_integer().not_null())
                    .col(ColumnDef::new(Permissions::PermissionBits).big_integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-resource-permission")
                            .from(Permissions::Table, Permissions::ResourceInternalId)
                            .to(Resources::Table, Resources::InternalId)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade)
                    )
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Permissions::Table).to_owned())
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
pub enum Permissions {
    Table,
    ResourceInternalId,
    SubjectInternalId,
    SubjectType,
    PermissionBits,
}