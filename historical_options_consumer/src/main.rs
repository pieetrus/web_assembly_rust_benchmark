pub mod kaffka;
pub mod processing;
pub mod services;
pub mod models;
use anyhow::Result;
use historical_options_consumer::{
    models::{types::HistoricalOption, HistoricalOptionsResponse}
    , processing::options_processor::OptionsProcessor
    // , services::native_filter_options
};
use std::path::Path;
use std::fs::File;
use std::io::Read;
use anyhow::Context;
use serde_json;
use rand::seq::SliceRandom;
use rand::thread_rng;

#[tokio::main]
async fn main() -> Result<()> {
    let wasm_file = Path::new("wasm-filter/target/wasm32-unknown-unknown/release/wasm_filter.wasm");
    let mut options_processor = OptionsProcessor::new(wasm_file).unwrap();

    let sample_data = prepare_sample_data(100_000).expect("Failed to prepare sample data");

    if let Err(err) = options_processor.process_options(&sample_data) {
        // Handle the error here
        eprintln!("Failed to process options: {}", err);
    }

    Ok(())
}


pub fn prepare_sample_data(size: usize) -> Result<Vec<HistoricalOption>> {
    let mut file = File::open("sample_data.json").context("Failed to open sample data file")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents).context("Failed to read sample data file")?;
    
    let response: HistoricalOptionsResponse = serde_json::from_str(&contents)
        .context("Failed to parse sample data")?;
    
    let mut data = response.data;
    let mut rng = thread_rng();

    if data.len() < size {
        // Jeśli potrzebujemy więcej danych, powielamy istniejące
        while data.len() < size {
            data.extend(data.clone());
        }
    }
    
    // Mieszamy dane i wybieramy losowy podzbiór o żądanym rozmiarze
    data.shuffle(&mut rng);
    data.truncate(size);

    Ok(data)
}