pub mod database {
    use dotenv::dotenv;
    use sea_orm::Database;

    use migration::{Migrator, MigratorTrait};

    pub async fn setup_database() -> Result<(), anyhow::Error> {
        dotenv().ok();

        let database_url = std::env::var("DATABASE_URL").unwrap();

        let connection = Database::connect(&database_url).await?;
        Migrator::up(&connection, None).await?;


        Ok(())
    }
}