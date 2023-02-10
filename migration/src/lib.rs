pub use sea_orm_migration::prelude::*;

mod m2022_0524_create_family;
mod m2022_0531_create_post;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m2022_0524_create_family::Migration),
            Box::new(m2022_0531_create_post::Migration),
        ]
    }
}
