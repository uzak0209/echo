pub use sea_orm_migration::prelude::*;

mod create_posts_table;
mod create_users_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(create_users_table::Migration),
            Box::new(create_posts_table::Migration),
        ]
    }
}
