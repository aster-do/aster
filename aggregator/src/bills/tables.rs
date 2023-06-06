use log::{debug, info};
use sqlx::{migrate, PgPool};

pub async fn run_migrations(pool: &PgPool) -> Result<(), anyhow::Error> {
    debug!("Running migrations!");
    migrate!().run(pool).await?;

    info!("We're good to go 🚀!");
    Ok(())
}
