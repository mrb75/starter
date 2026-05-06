use clap::{ArgAction, Parser, Subcommand};
use shared::{
    args::BaseArgs,
    types::{DatabaseConfig, DatabaseKind, DatabasePurpose, DatabaseType},
};

#[derive(Parser)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Rocket {
        #[clap(flatten)]
        base: BaseArgs,

        #[arg(long, default_value = "8000")]
        port: u16,

        // Multiple databases with purpose
        #[arg(long, value_parser = parse_database, action = ArgAction::Append)]
        database: Vec<DatabaseConfig>,
    },
}

fn parse_database(s: &str) -> Result<DatabaseConfig, String> {
    // Format: kind[:purpose][@version]
    // Examples:
    //   postgres                          -> primary
    //   postgres:primary                  -> explicit primary
    //   redis:cache                       -> Redis for caching
    //   redis:sessions                    -> Redis for sessions
    //   postgres:primary@16               -> PostgreSQL 16 as primary
    //   redis:queue                       -> Redis for queue

    let parts: Vec<&str> = s.split(':').collect();
    let kind_part = parts[0];
    let purpose_part = parts.get(1).copied().unwrap_or("primary");
    let version_part = parts.get(2).and_then(|p| p.strip_prefix('@'));

    let kind = DatabaseKind::from_str(kind_part)
        .ok_or_else(|| format!("Unknown database kind: {}", kind_part))?;

    let purpose = match purpose_part {
        "primary" => DatabasePurpose::Primary,
        "cache" => DatabasePurpose::Cache,
        "sessions" => DatabasePurpose::Sessions,
        "queue" => DatabasePurpose::Queue,
        "analytics" => DatabasePurpose::Analytics,
        "logging" => DatabasePurpose::Logging,
        custom => DatabasePurpose::Custom(custom.to_string()),
    };

    Ok(DatabaseConfig {
        db_type: DatabaseType {
            kind,
            version: version_part.map(|v| v.to_string()),
        },
        purpose,
        custom_name: None,
    })
}
