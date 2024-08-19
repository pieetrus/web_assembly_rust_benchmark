use anyhow::{Result, Context};
use std::path::Path;
use std::fs::File;
use std::io::Write;
use crate::kaffka::KafkaConsumer;
use crate::processing::options_processor::OptionsProcessor;
use crate::models::types::HistoricalOptionsResponse;

pub async fn consume_and_process_options_wasm() -> Result<()> {
    let kafka_consumer = KafkaConsumer::new(
        "localhost:29092",
        "historical_options_group",
        "historical_options"
    )?;

    let wasm_file = Path::new("wasm-filter/target/wasm32-unknown-unknown/release/wasm_filter.wasm");
    let mut options_processor = OptionsProcessor::new(wasm_file)?;

    println!("Consumer started. Waiting for messages...");

    // Create a file to save the data
    let mut file = File::create("sample_data.json").context("Failed to create sample data file")?;

    loop {
        match kafka_consumer.receive_message().await {
            Ok(payload) => {
                println!("Received new message. Processing...");
                let json_data: HistoricalOptionsResponse = serde_json::from_slice(&payload)
                    .context("Failed to parse historical options data")?;
                
                // Save the received data to the file
                file.write_all(&payload).context("Failed to write data to file")?;
                file.flush().context("Failed to flush file")?;

                let filtered_options = options_processor.process_options(&json_data.data)?;
                options_processor.print_filtered_options(&filtered_options);
            }
            Err(e) => {
                eprintln!("Error receiving message: {}", e);
                tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
            }
        }
    }
}