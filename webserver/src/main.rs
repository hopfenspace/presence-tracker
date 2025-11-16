use std::error::Error;
use std::net::SocketAddr;

use clap::Parser;
use galvyn::Galvyn;
use galvyn::GalvynSetup;
use galvyn::core::DatabaseSetup;
use galvyn::core::re_exports::rorm;
use galvyn::rorm::Database;
use galvyn::rorm::DatabaseConfiguration;
use galvyn::rorm::config::DatabaseConfig;
use tracing::instrument;
use tracing::level_filters::LevelFilter;
use tracing_subscriber::EnvFilter;
use tracing_subscriber::Layer;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

use crate::cli::Cli;
use crate::cli::Command;
use crate::config::DB;
use crate::config::LISTEN_ADDRESS;
use crate::config::LISTEN_PORT;

mod cli;
pub mod config;
pub mod http;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    tracing_subscriber::registry()
        .with(EnvFilter::try_from_default_env().unwrap_or(EnvFilter::new("INFO")))
        .with(tracing_forest::ForestLayer::default().with_filter(LevelFilter::DEBUG))
        .init();

    galvyn::panic_hook::set_panic_hook();

    let cli = Cli::parse();

    match cli.command {
        Command::Start => start().await?,
        Command::MakeMigrations => make_migrations().await?,
        Command::Migrate => migrate().await?,
    }

    Ok(())
}

#[instrument]
async fn start() -> Result<(), Box<dyn Error>> {
    Galvyn::builder(GalvynSetup::default())
        .register_module::<Database>(DatabaseSetup::Custom(DatabaseConfiguration::new(
            DB.clone(),
        )))
        .init_modules()
        .await?
        .add_routes(http::initialize_routes())
        .start(SocketAddr::from((
            *LISTEN_ADDRESS.get(),
            *LISTEN_PORT.get(),
        )))
        .await?;

    Ok(())
}

async fn make_migrations() -> Result<(), Box<dyn Error>> {
    use std::io::Write;

    /// Temporary file to store models in
    const MODELS: &str = "/tmp/.models.json";

    let mut file = std::fs::File::create(MODELS)?;
    rorm::write_models(&mut file)?;
    file.flush()?;

    rorm::cli::make_migrations::run_make_migrations(
        rorm::cli::make_migrations::MakeMigrationsOptions {
            models_file: MODELS.to_string(),
            migration_dir: "/migrations".to_string(),
            name: None,
            non_interactive: false,
            warnings_disabled: false,
        },
    )?;

    std::fs::remove_file(MODELS)?;
    Ok(())
}

async fn migrate() -> Result<(), Box<dyn Error>> {
    rorm::cli::migrate::run_migrate_custom(
        DatabaseConfig {
            driver: DB.clone(),
            last_migration_table_name: None,
        },
        "/migrations".to_string(),
        false,
        None,
    )
    .await?;
    Ok(())
}
