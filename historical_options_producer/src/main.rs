use services::produce_historical_options;
use anyhow::Result;

mod services;
mod api;
mod kaffka;

#[tokio::main]
async fn main() -> Result<()> {
    produce_historical_options().await?;
    Ok(())
}