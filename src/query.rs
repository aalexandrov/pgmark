//! The `query` CLI command.

use ascii_table::{Align, AsciiTable};
use tokio_postgres::{Client, Column, Row};

use crate::cli;
use crate::error;

/// Run a SQL query and return the results.
#[derive(clap::Args, Debug)]
#[command(version, about, long_about = None)]
pub struct Query {
    sql: String,
}

#[derive(thiserror::Error, Debug)]
#[error(transparent)]
pub enum Error {
    #[error("unsupported column type `{0}` when displaying")]
    UnsupportedType(String),
}

impl cli::Run for Query {
    async fn run(self, client: Client) -> Result<(), error::Error> {
        // Now we can execute a simple statement that just returns its parameter.
        // let rows = client.query("SELECT $1::TEXT", &[&"hello world"]).await?; TODO: error
        let rows = client.query(&self.sql, &[]).await?;

        if rows.is_empty() {
            println!("0 results");
        } else {
            print_rows(&rows)?;
        }

        Ok(())
    }
}

fn print_rows(rows: &[Row]) -> Result<(), Error> {
    use std::fmt::Display;

    let mut table = AsciiTable::default();
    table.set_max_width(200);

    // Configure columns
    for (idx, col) in columns(rows).enumerate() {
        let header = col.name();
        let align = alignment(col.type_().name());
        table.column(idx).set_header(header).set_align(align);
    }

    // Collect data
    let mut data: Vec<Vec<Box<dyn Display>>> = Vec::with_capacity(rows.len());

    for row in rows {
        let mut data_row: Vec<Box<dyn Display>> = vec![];
        for (idx, col) in row.columns().iter().enumerate() {
            let val: Box<dyn Display> = match col.type_().name() {
                "int2" => Box::new(row.get::<_, i16>(idx)),
                "int4" => Box::new(row.get::<_, i32>(idx)),
                "int8" => Box::new(row.get::<_, i64>(idx)),
                "float4" => Box::new(row.get::<_, f32>(idx)),
                "float8" => Box::new(row.get::<_, f64>(idx)),
                "bpchar" => Box::new(row.get::<_, &str>(idx)),
                "varchar" | "text" => Box::new(row.get::<_, &str>(idx)),
                "unknown" => Box::new("NULL"), // TODO: might be wrong?
                name => {
                    dbg!(col.type_());
                    return Err(Error::UnsupportedType(name.into()));
                }
            };
            data_row.push(val);
        }
        data.push(data_row);
    }

    table.print(data);

    Ok(())
}

fn columns(rows: &[Row]) -> impl Iterator<Item = &Column> {
    rows.last().map(Row::columns).into_iter().flatten()
}

fn alignment(type_name: &str) -> Align {
    let align_right = ["int2", "int4", "int8", "float4", "float8", "oid"];
    if align_right.contains(&type_name) {
        Align::Right
    } else {
        Align::Left
    }
}
