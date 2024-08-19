pub mod kaffka;
pub mod processing;
pub mod services;
pub mod models;
use anyhow::Result;
use historical_options_consumer::services::consume_and_process_options_wasm;

#[tokio::main]
async fn main() -> Result<()> {
    consume_and_process_options_wasm().await?;
    Ok(())
}