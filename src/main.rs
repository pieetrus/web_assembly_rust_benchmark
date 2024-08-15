mod services;
mod models;

use services::{consumer, producer};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    producer::get_historical_options_information().await?;
    
    consumer::consume()?;

    Ok(())
}