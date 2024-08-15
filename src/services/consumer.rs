use std::{error::Error, fs, path::PathBuf};
use crate::models::types::{HistoricalOption, HistoricalOptionsResponse};
use crate::services::wasm_handler::WasmHandler;
use anyhow::Result;

/// Load and parse the JSON file containing historical options data
fn load_historical_options(file_path: &str) -> Result<HistoricalOptionsResponse> {
    let file_content = fs::read_to_string(file_path)?;
    let json_data: HistoricalOptionsResponse = serde_json::from_str(&file_content)?;
    Ok(json_data)
}

/// Process historical options using the WebAssembly module
fn process_historical_options(wasm_handler: &mut WasmHandler, options: &[HistoricalOption]) -> Result<Vec<HistoricalOption>> {
    let options_json = serde_json::to_vec(options)?;
    let filtered_json = wasm_handler.filter_historical_options(&options_json)?;
    let filtered_options: Vec<HistoricalOption> = serde_json::from_slice(&filtered_json)?;
    Ok(filtered_options)
}

/// Print filtered historical options
fn print_filtered_options(options: &[HistoricalOption]) {
    println!("Filtered Historical Options:");
    for option in options {
        println!("{:#?}", option);
    }
}

pub fn consume() -> Result<(), Box<dyn Error>> {
    // Initialize WasmHandler
    let wasm_file = PathBuf::from("wasm-filter/target/wasm32-unknown-unknown/release/wasm_filter.wasm");
    let mut wasm_handler = WasmHandler::new(&wasm_file)?;

    // Load and parse historical options
    let json_data = load_historical_options("historical_options_information.json")?;

    // Process historical options using WebAssembly
    let filtered_options = process_historical_options(&mut wasm_handler, &json_data.data)?;

    // Print results
    print_filtered_options(&filtered_options);

    Ok(())
}