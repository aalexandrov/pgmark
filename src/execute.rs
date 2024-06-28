//! The `execute` CLI command.

use tokio_postgres::Client;

use crate::cli;
use crate::error;

/// Run a SQL query and return the results.
#[derive(clap::Args, Debug)]
#[command(version, about, long_about = None)]
pub struct Execute {
    sql: String,
}

impl cli::Run for Execute {
    async fn run(self, client: Client) -> Result<(), error::Error> {
        let rows_changed = client.execute(&self.sql, &[]).await?;
        println!("{rows_changed} rows changed");
        Ok(())
    }
}
