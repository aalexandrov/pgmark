use std::fmt::Write;
use tokio_postgres::Client;

use crate::error;

/// A command interface.
pub trait Run {
    /// Run the command.
    async fn run(self, client: Client) -> Result<(), error::Error>;
}

#[derive(clap::Parser, Debug)]
pub struct Database {
    #[arg(long, env = "PGHOST")]
    host: Option<String>,
    #[arg(long, env = "PGPORT")]
    port: Option<u16>,
    #[arg(long, env = "PGUSER")]
    user: Option<String>,
    #[arg(long, env = "PGPASSWORD")]
    password: Option<String>,
    #[arg(long, env = "PGDATABASE")]
    database: Option<String>,
}

impl Database {
    pub fn into_connection(self: Database) -> Result<String, std::fmt::Error> {
        let mut buffer = String::new();
        let space = |buffer: &str| if buffer.is_empty() { "" } else { " " };

        if let Some(host) = self.host {
            write!(buffer, "{}host={host}", space(&buffer))?;
        }
        if let Some(port) = self.port {
            write!(buffer, "{}port={port}", space(&buffer))?;
        }
        if let Some(user) = self.user {
            write!(buffer, "{}user={user}", space(&buffer))?;
        }
        if let Some(password) = self.password {
            write!(buffer, "{}password={password}", space(&buffer))?;
        }

        Ok(buffer)
    }
}
