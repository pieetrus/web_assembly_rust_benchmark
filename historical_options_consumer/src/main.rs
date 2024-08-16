mod kaffka;
mod processing;
mod models;
mod services;
use anyhow::Result;
use services::consume_and_process_options;

#[tokio::main]
async fn main() -> Result<()> {
    consume_and_process_options().await?;
    Ok(())
}