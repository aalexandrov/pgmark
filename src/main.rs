use clap::Parser;
use tokio_postgres::{Client, NoTls};

mod cli;
mod error;
mod execute;
mod load;
mod prepare;
mod query;

use crate::cli::Run;

/// does testing things.
#[derive(clap::Subcommand, Debug)]
pub enum Command {
    Query(query::Query),
    Execute(execute::Execute),
    Prepare(prepare::Prepare),
    Load(load::Load),
}

impl cli::Run for Command {
    async fn run(self, client: Client) -> Result<(), error::Error> {
        use Command::*;
        match self {
            Query(cmd) => cmd.run(client).await,
            Execute(cmd) => cmd.run(client).await,
            Prepare(cmd) => cmd.run(client).await,
            Load(cmd) => cmd.run(client).await,
        }
    }
}

/// Run a SQL command through [`tokio_postgres`].
#[derive(clap::Parser, Debug)]
pub struct Cli {
    #[command(flatten)]
    pub database: cli::Database,
    #[command(subcommand)]
    pub command: Command,
}

async fn run() -> Result<(), error::Error> {
    let cli = Cli::try_parse()?;

    // Connect to the database.
    let (client, connection) = {
        let config = cli.database.into_connection()?;
        tokio_postgres::connect(&config, NoTls).await?
    };

    // The connection object performs the actual communication with the database,
    // so spawn it off to run on its own.
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    cli.command.run(client).await
}

#[tokio::main] // By default, tokio_postgres uses the tokio crate as its runtime.
async fn main() {
    if let Err(err) = run().await {
        eprintln!("{err}");
        std::process::exit(-1);
    }
}
