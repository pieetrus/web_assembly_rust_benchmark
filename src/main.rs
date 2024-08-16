mod api;
mod kaffka;
mod processing;
mod models;
mod services;

use anyhow::Result;
use services::{produce_historical_options, consume_and_process_options};

#[tokio::main]
async fn main() -> Result<()> {
    produce_historical_options().await?;
    consume_and_process_options().await?;
    Ok(())
}