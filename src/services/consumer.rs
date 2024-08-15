use std::{error::Error, fs, path::PathBuf};
use crate::models::types::{Incident, TrafficIncidentResponse};
use crate::services::wasm_handler::WasmHandler;
use anyhow::Result;

/// Load and parse the JSON file containing traffic incidents
fn load_incidents(file_path: &str) -> Result<TrafficIncidentResponse> {
    let file_content = fs::read_to_string(file_path)?;
    let json_data: TrafficIncidentResponse = serde_json::from_str(&file_content)?;
    Ok(json_data)
}

/// Process incidents using the WebAssembly module
fn process_incidents(wasm_handler: &mut WasmHandler, incidents: &[Incident]) -> Result<Vec<Incident>> {
    let incidents_json = serde_json::to_vec(incidents)?;
    let filtered_json = wasm_handler.filter_incidents(&incidents_json)?;
    let filtered_incidents: Vec<Incident> = serde_json::from_slice(&filtered_json)?;
    Ok(filtered_incidents)
}

/// Print filtered incidents
fn print_filtered_incidents(incidents: &[Incident]) {
    println!("Filtered Incidents (excluding roadworks):");
    for incident in incidents {
        println!("{:#?}", incident);
    }
}

pub fn consume() -> Result<(), Box<dyn Error>> {
    // Initialize WasmHandler
    let wasm_file = PathBuf::from("wasm-filter/target/wasm32-unknown-unknown/release/wasm_filter.wasm");
    let mut wasm_handler = WasmHandler::new(&wasm_file)?;

    // Load and parse incidents
    let json_data = load_incidents("traffic_incidents.json")?;

    // Process incidents using WebAssembly
    let filtered_incidents = process_incidents(&mut wasm_handler, &json_data.incidents)?;

    // Print results
    print_filtered_incidents(&filtered_incidents);

    Ok(())
}