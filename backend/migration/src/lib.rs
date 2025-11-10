pub use sea_orm_migration::prelude::*;

mod create_posts_table;
mod create_users_table;
mod add_user_credentials;
mod create_reactions_table;
mod add_refresh_token_to_users;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(create_users_table::Migration),
            Box::new(create_posts_table::Migration),
            Box::new(add_user_credentials::Migration),
            Box::new(create_reactions_table::Migration),
            Box::new(add_refresh_token_to_users::Migration),
        ]
    }
}
