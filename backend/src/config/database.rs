pub mod database {
    use sea_orm::Database;

    use migration::{Migrator, MigratorTrait};

    pub async fn setup_database(database_url: &str) -> Result<(), anyhow::Error> {
        let connection = Database::connect(&database_url.to_owned()).await?;
        Migrator::up(&connection, None).await?;


        Ok(())
    }
}