//! The `prepare` CLI command.
use std::io::stdin;

use bytes::BytesMut;
use futures::{pin_mut, SinkExt};
use tokio_postgres::Client;

use crate::cli;
use crate::error;

/// Run a SQL query and return the results.
#[derive(clap::Args, Debug)]
#[command(version, about, long_about = None)]
pub struct Load {
    sql: String,
}

impl cli::Run for Load {
    async fn run(self, client: Client) -> Result<(), error::Error> {
        let sink = client.copy_in(&self.sql).await?;

        pin_mut!(sink);

        let mut data = String::new();
        loop {
            let len = data.len();
            stdin().read_line(&mut data).expect("Failed to read line");
            if data[len..].trim() == "\\." {
                data.truncate(len);
                break;
            }
        }

        let bytes = BytesMut::from(data.as_bytes());
        sink.send(bytes).await?;

        let rows = sink.finish().await?;
        println!("stmt.finish() = {}", rows);

        Ok(())
    }
}
