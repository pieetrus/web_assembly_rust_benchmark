use anyhow::{Result, Context};
use std::path::Path;
use crate::kaffka::KafkaConsumer;
use crate::processing::options_processor::OptionsProcessor;
use crate::models::types::HistoricalOptionsResponse;

pub async fn consume_and_process_options() -> Result<()> {
    let kafka_consumer = KafkaConsumer::new(
        "localhost:29092",
        "historical_options_group",
        "historical_options"
    )?;

    let wasm_file = Path::new("wasm-filter/target/wasm32-unknown-unknown/release/wasm_filter.wasm");
    let mut options_processor = OptionsProcessor::new(wasm_file)?;

    println!("Consumer started. Waiting for messages...");

    loop {
        match kafka_consumer.receive_message().await {
            Ok(payload) => {
                println!("Received new message. Processing...");
                let json_data: HistoricalOptionsResponse = serde_json::from_slice(&payload)
                    .context("Failed to parse historical options data")?;

                let filtered_options = options_processor.process_options(&json_data.data)?;
                options_processor.print_filtered_options(&filtered_options);
            }
            Err(e) => {
                eprintln!("Error receiving message: {}", e);
                // Optionally add a short delay to avoid tight loop on persistent errors
                tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
            }
        }
    }
}
