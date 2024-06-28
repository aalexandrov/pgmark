//! Errors types.

use crate::query;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error(transparent)]
    Clap(#[from] clap::Error),
    #[error(transparent)]
    Format(#[from] std::fmt::Error),
    #[error(transparent)]
    Query(#[from] query::Error),
    #[error("database error (code {}): {0}", .0.code().map_or("unknown", |c| c.code()))]
    Tokio(#[from] tokio_postgres::Error),
}
