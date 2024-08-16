use anyhow::Result;
use crate::{api::MockServerClient, kaffka::KafkaProducer};

pub async fn produce_historical_options() -> Result<()> {
    dotenv::dotenv().ok();
    let mock_client = MockServerClient::new();
    let kafka_producer = KafkaProducer::new("localhost:29092")?;

    let symbol = "IBM";
    let historical_data = mock_client.get_historical_options().await?;
    kafka_producer.send_message("historical_options", symbol, &historical_data).await?;
    
    println!("Produced message");
    
    Ok(())
}