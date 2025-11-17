//! Definitions of the CLI arguments

use clap::Parser;
use clap::Subcommand;

/// The cli
#[derive(Parser)]
pub struct Cli {
    /// The available subcommands
    #[clap(subcommand)]
    pub command: Command,
}

/// All available commands
#[derive(Subcommand)]
pub enum Command {
    /// Start the server
    Start,
    /// Run the migrations on the database
    Migrate,
    /// Create new migrations
    MakeMigrations {
        /// Target directory for the migrations
        migration_dir: String,
    },
}
