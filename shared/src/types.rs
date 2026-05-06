// trait/src/types.rs
use std::fmt;

#[derive(Debug, Clone, PartialEq, clap::ValueEnum)]
pub enum AuthType {
    Jwt,
    Sessions,
    OAuth,
}

#[derive(Debug, Clone)]
pub struct DatabaseConfig {
    pub db_type: DatabaseType,
    pub purpose: DatabasePurpose,
    pub custom_name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DatabasePurpose {
    Primary,        // Main application data
    Cache,          // Redis for caching
    Sessions,       // Session storage
    Queue,          // Message queue (Redis, RabbitMQ)
    Analytics,      // Analytics database
    Logging,        // Log storage
    Custom(String), // User-defined purpose
}

impl DatabasePurpose {
    pub fn as_str(&self) -> &str {
        match self {
            DatabasePurpose::Primary => "primary",
            DatabasePurpose::Cache => "cache",
            DatabasePurpose::Sessions => "sessions",
            DatabasePurpose::Queue => "queue",
            DatabasePurpose::Analytics => "analytics",
            DatabasePurpose::Logging => "logging",
            DatabasePurpose::Custom(s) => s,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct DatabaseType {
    pub kind: DatabaseKind,
    pub version: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DatabaseKind {
    Postgres,
    MySql,
    Sqlite,
    MongoDb,
    Redis,
    DynamoDb,
    Cassandra,
    ClickHouse,
}

impl DatabaseKind {
    pub fn default_port(&self) -> u16 {
        match self {
            DatabaseKind::Postgres => 5432,
            DatabaseKind::MySql => 3306,
            DatabaseKind::Sqlite => 0,
            DatabaseKind::MongoDb => 27017,
            DatabaseKind::Redis => 6379,
            DatabaseKind::DynamoDb => 8000,
            DatabaseKind::Cassandra => 9042,
            DatabaseKind::ClickHouse => 8123,
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "postgres" | "postgre" | "pg" => Some(DatabaseKind::Postgres),
            "mysql" => Some(DatabaseKind::MySql),
            "sqlite" => Some(DatabaseKind::Sqlite),
            "mongodb" | "mongo" => Some(DatabaseKind::MongoDb),
            "redis" => Some(DatabaseKind::Redis),
            "dynamodb" | "dynamo" => Some(DatabaseKind::DynamoDb),
            "cassandra" => Some(DatabaseKind::Cassandra),
            "clickhouse" => Some(DatabaseKind::ClickHouse),
            _ => None,
        }
    }
}

impl fmt::Display for DatabaseKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DatabaseKind::Postgres => write!(f, "PostgreSQL"),
            DatabaseKind::MySql => write!(f, "MySQL"),
            DatabaseKind::Sqlite => write!(f, "SQLite"),
            DatabaseKind::MongoDb => write!(f, "MongoDB"),
            DatabaseKind::Redis => write!(f, "Redis"),
            DatabaseKind::DynamoDb => write!(f, "DynamoDB"),
            DatabaseKind::Cassandra => write!(f, "Cassandra"),
            DatabaseKind::ClickHouse => write!(f, "ClickHouse"),
        }
    }
}
