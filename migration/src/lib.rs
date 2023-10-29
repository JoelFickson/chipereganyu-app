pub use sea_orm_migration::prelude::*;

mod m20231029_193540_users_table;
mod m20231029_200844_groups_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20231029_193540_users_table::Migration),
            Box::new(m20231029_200844_groups_table::Migration),
        ]
    }
}
