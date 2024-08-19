pub mod kaffka;
pub mod processing;
pub mod services;
pub mod models;
use anyhow::Result;

use services::consume_and_process_options;

#[tokio::main]
async fn main() -> Result<()> {
    consume_and_process_options().await?;
    Ok(())
}