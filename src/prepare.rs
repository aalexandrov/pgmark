//! The `prepare` CLI command.

use super::*;
use tokio_postgres::Client;

/// Run a SQL query and return the results.
#[derive(clap::Args, Debug)]
#[command(version, about, long_about = None)]
pub struct Prepare {
    sql: String,
}

impl cli::Run for Prepare {
    async fn run(self, client: Client) -> Result<(), error::Error> {
        let stmt = client.prepare(&self.sql).await?;
        println!("stmt.params() = {:#?}", stmt.params());
        println!("stmt.columns() = {:#?}", stmt.columns());
        Ok(())
    }
}
