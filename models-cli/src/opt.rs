use std::path::Path;

use anyhow::Result;
use sqlx::migrate::{
    MigrateError,
    Migrator,
};
use structopt::StructOpt;
#[derive(StructOpt, Debug)]
pub struct Opt {
    #[structopt(subcommand)]
    pub command: Command,
}

#[derive(StructOpt, Debug)]
pub enum Command {
    #[structopt(alias = "db")]
    Database(DatabaseOpt),

    #[structopt(alias = "mig")]
    Migrate(MigrateOpt),

    #[structopt(alias = "gen")]
    Generate(GenerateOpt),
}

/// Group of commands for creating and dropping your database.
#[derive(StructOpt, Debug)]
pub struct DatabaseOpt {
    #[structopt(subcommand)]
    pub command: DatabaseCommand,
}

#[derive(StructOpt, Debug)]
pub enum DatabaseCommand {
    /// Creates the database specified in your DATABASE_URL.
    Create {
        /// Location of the DB, by default will be read from the DATABASE_URL
        /// env var
        #[structopt(long, short = "D", env)]
        database_url: String,
    },

    /// Drops the database specified in your DATABASE_URL.
    Drop {
        /// Automatic confirmation. Without this option, you will be prompted
        /// before dropping your database.
        #[structopt(short)]
        yes: bool,

        /// Location of the DB, by default will be read from the DATABASE_URL
        /// env var
        #[structopt(long, short = "D", env)]
        database_url: String,
    },

    /// Drops the database specified in your DATABASE_URL, re-creates it, and
    /// runs any pending migrations.
    Reset {
        /// Automatic confirmation. Without this option, you will be prompted
        /// before dropping your database.
        #[structopt(short)]
        yes: bool,

        /// Path to folder containing migrations.
        #[structopt(long, default_value = "migrations")]
        source: String,

        /// Location of the DB, by default will be read from the DATABASE_URL
        /// env var
        #[structopt(long, short = "D", env)]
        database_url: String,
    },

    /// Creates the database specified in your DATABASE_URL and runs any pending
    /// migrations.
    Setup {
        /// Path to folder containing migrations.
        #[structopt(long, default_value = "migrations")]
        source: String,

        /// Location of the DB, by default will be read from the DATABASE_URL
        /// env var
        #[structopt(long, short = "D", env)]
        database_url: String,
    },
}

/// Group of commands for creating and running migrations.
#[derive(StructOpt, Debug)]
pub struct MigrateOpt {
    /// Path to folder containing migrations.
    #[structopt(long, default_value = "migrations")]
    pub source: String,

    #[structopt(subcommand)]
    pub command: MigrateCommand,
}
/// Commands related to automatic migration generation.
#[derive(StructOpt, Debug)]
pub struct GenerateOpt {
    /// Location of the DB, by default will be read from the DATABASE_URL env
    /// var
    #[structopt(long, short = "D", env)]
    pub database_url: String,
    /// Path to folder containing migrations.
    #[structopt(long, default_value = "migrations")]
    pub source: String,
    /// Used to filter through the models to execute.
    #[structopt(long)]
    pub table: Option<String>,
    /// Used to generate a down migrations along with up migrations.
    #[structopt(short)]
    pub reversible: bool,
}

impl GenerateOpt {
    pub async fn validate(&self) -> Result<()> {
        url::Url::parse(&self.database_url)?;
        let migrator = Migrator::new(Path::new(&self.source)).await?;
        for migration in migrator.iter() {
            if migration.migration_type.is_reversible() != self.reversible {
                Err(MigrateError::InvalidMixReversibleAndSimple)?
            }
        }

        Ok(())
    }
}

#[derive(StructOpt, Debug)]
pub enum MigrateCommand {
    /// Create a new migration with the given description,
    /// and the current time as the version.
    Add {
        description: String,

        /// If true, creates a pair of up and down migration files with same
        /// version else creates a single sql file
        #[structopt(short)]
        reversible: bool,
    },

    /// Run all pending migrations.
    Run {
        /// List all the migrations to be run without applying
        #[structopt(long)]
        dry_run: bool,

        /// Ignore applied migrations that missing in the resolved migrations
        #[structopt(long)]
        ignore_missing: bool,

        /// Location of the DB, by default will be read from the DATABASE_URL
        /// env var
        #[structopt(long, short = "D", env)]
        database_url: String,
    },

    /// Revert the latest migration with a down file.
    Revert {
        /// List the migration to be reverted without applying
        #[structopt(long)]
        dry_run: bool,

        /// Ignore applied migrations that missing in the resolved migrations
        #[structopt(long)]
        ignore_missing: bool,

        /// Location of the DB, by default will be read from the DATABASE_URL
        /// env var
        #[structopt(long, short = "D", env)]
        database_url: String,
    },

    /// List all available migrations.
    Info {
        /// Location of the DB, by default will be read from the DATABASE_URL
        /// env var
        #[structopt(long, env)]
        database_url: String,
    },

    /// Generate a `build.rs` to trigger recompilation when a new migration is
    /// added.
    ///
    /// Must be run in a Cargo project root.
    BuildScript {
        /// Overwrite the build script if it already exists.
        #[structopt(long)]
        force: bool,
    },
}
