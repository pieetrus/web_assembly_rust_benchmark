mod services;
mod models;

use services::{consumer, producer};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    producer::get_traffic_incidents().await?;
    
    consumer::consume()?;

    Ok(())
}