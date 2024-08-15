use std::{error::Error, fs, path::PathBuf};
use crate::models::types::{Incident, TrafficIncidentResponse};
use wasmtime::*;
use anyhow::Result;

pub fn consume() -> Result<(), Box<dyn Error>> {
    let wasm_file = PathBuf::from("wasm-filter/target/wasm32-unknown-unknown/release/wasm_filter.wasm");

    let file_content = fs::read_to_string("traffic_incidents.json")?;
    let json_data: TrafficIncidentResponse = serde_json::from_str(&file_content)?;

    let engine = Engine::default();
    let module = Module::from_file(&engine, &wasm_file)?;
    let mut store = Store::new(&engine, ());

    let instance = Instance::new(&mut store, &module, &[])?;

    let memory = instance.get_memory(&mut store, "memory").ok_or("Failed to get memory")?;
    let alloc = instance.get_typed_func::<i32, i32>(&mut store, "alloc")?;
    let dealloc = instance.get_typed_func::<(i32, i32), ()>(&mut store, "dealloc")?;
    let filter_incidents = instance.get_typed_func::<(i32, i32), i32>(&mut store, "filter_incidents")?;
    let get_result_len = instance.get_typed_func::<(), i32>(&mut store, "get_result_len")?;

    let incidents_json = serde_json::to_vec(&json_data.incidents)?;
    let incidents_len = incidents_json.len();
    let incidents_ptr = alloc.call(&mut store, incidents_len as i32)?;
    memory.write(&mut store, incidents_ptr as usize, &incidents_json)?;

    let result_ptr = filter_incidents.call(&mut store, (incidents_ptr, incidents_len as i32))?;
    let result_len = get_result_len.call(&mut store, ())?;

    let mut result = vec![0u8; result_len as usize];
    memory.read(&store, result_ptr as usize, &mut result)?;

    dealloc.call(&mut store, (incidents_ptr, incidents_len as i32))?;

    let filtered_incidents: Vec<Incident> = serde_json::from_slice(&result)?;

    println!("Filtered Incidents (excluding roadworks):");
    for incident in filtered_incidents {
        println!("{:#?}", incident);
    }

    Ok(())
}