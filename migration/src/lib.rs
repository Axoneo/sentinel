pub use sea_orm_migration::prelude::*;

mod m20251122_070217_create_namespaces;
mod m20251122_070408_create_resources;
mod m20251122_090721_create_users;
mod m20251122_100616_create_groups;
mod m20251122_115332_create_permissions;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20251122_070217_create_namespaces::Migration),
            Box::new(m20251122_070408_create_resources::Migration),
            Box::new(m20251122_090721_create_users::Migration),
            Box::new(m20251122_100616_create_groups::Migration),
            Box::new(m20251122_115332_create_permissions::Migration),
        ]
    }
}
