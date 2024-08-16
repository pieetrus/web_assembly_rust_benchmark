use anyhow::{Result, Context};
use crate::{api::AlphaVantageClient, kaffka::KafkaProducer};

pub async fn produce_historical_options() -> Result<()> {
    dotenv::dotenv().ok();
    let api_key = std::env::var("API_KEY")
        .context("API_KEY must be set in env variables")?;
    let alpha_vantage = AlphaVantageClient::new(api_key);
    let kafka_producer = KafkaProducer::new("localhost:29092")?;

    let symbol = "IBM";
    let historical_data = alpha_vantage.get_historical_options(symbol).await?;
    kafka_producer.send_message("historical_options", symbol, &historical_data).await?;

    Ok(())
}