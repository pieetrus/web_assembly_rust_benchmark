use criterion::{criterion_group, criterion_main, Criterion};
use historical_options_consumer::{
    models::{types::HistoricalOption, HistoricalOptionsResponse}, processing::options_processor::OptionsProcessor
};
use std::path::Path;
use std::fs::File;
use std::io::Read;
use anyhow::{Result, Context};
use serde_json;

fn benchmark_wasm_filter(c: &mut Criterion) {
    let sample_data: Vec<HistoricalOption> = prepare_sample_data().expect("Failed to prepare sample data");
    
    let wasm_file = Path::new("wasm-filter/target/wasm32-unknown-unknown/release/wasm_filter.wasm");
    let mut options_processor = OptionsProcessor::new(wasm_file).unwrap();

    c.bench_function("filter_historical_options", |b| {
        b.iter(|| {
            // Clone the sample data for each iteration to ensure consistent benchmarking
            let data_clone = sample_data.clone();
            options_processor.process_options(&data_clone).unwrap()
        })
    });
}

pub fn prepare_sample_data() -> Result<Vec<HistoricalOption>> {
    let mut file = File::open("sample_data.json").context("Failed to open sample data file")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents).context("Failed to read sample data file")?;

    let response: HistoricalOptionsResponse = serde_json::from_str(&contents)
        .context("Failed to parse sample data")?;

    Ok(response.data)
}

criterion_group!(benches, benchmark_wasm_filter);
criterion_main!(benches);