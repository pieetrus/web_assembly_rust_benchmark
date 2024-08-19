use criterion::{criterion_group, criterion_main, Criterion};
use historical_options_consumer::{
    models::{types::HistoricalOption, HistoricalOptionsResponse}, processing::options_processor::OptionsProcessor, services::native_filter_options
};
use std::path::Path;
use std::fs::File;
use std::io::Read;
use anyhow::{Result, Context};
use serde_json;
use criterion::BenchmarkId;
use rand::seq::SliceRandom;
use rand::thread_rng;

const DEFAULT_SAMPLE_DATA_SIZE : usize = 1000;

fn benchmark_wasm_filter(c: &mut Criterion) {
    let sample_data: Vec<HistoricalOption> = prepare_sample_data(DEFAULT_SAMPLE_DATA_SIZE).expect("Failed to prepare sample data");
    
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

fn benchmark_native_filter(c: &mut Criterion) {
    let sample_data: Vec<HistoricalOption> = prepare_sample_data(DEFAULT_SAMPLE_DATA_SIZE).expect("Failed to prepare sample data");

    c.bench_function("native_filter_historical_options", |b| {
        b.iter(|| {
            let data_clone = sample_data.clone();
            native_filter_options(&data_clone)
        })
    });
}


fn compare_filters(c: &mut Criterion) {
    let sizes = vec![100, 1000, 10000, 100000];
    
    let wasm_file = Path::new("wasm-filter/target/wasm32-unknown-unknown/release/wasm_filter.wasm");
    let mut options_processor = OptionsProcessor::new(wasm_file).unwrap();

    let mut group = c.benchmark_group("Filter Comparison");
    for size in sizes {
        let sample_data = prepare_sample_data(size).expect("Failed to prepare sample data");
        
        group.bench_with_input(BenchmarkId::new("WASM", size), &sample_data, |b, data| {
            b.iter(|| options_processor.process_options(data).unwrap());
        });
        group.bench_with_input(BenchmarkId::new("Native", size), &sample_data, |b, data| {
            b.iter(|| native_filter_options(data));
        });
    }
    group.finish();
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

// fn compare_filters_various_sizes(c: &mut Criterion) {
//     let sizes = vec![100, 1000, 10000];
    
//     let wasm_file = Path::new("wasm-filter/target/wasm32-unknown-unknown/release/wasm_filter.wasm");
//     let mut options_processor = OptionsProcessor::new(wasm_file).unwrap();

//     let mut group = c.benchmark_group("Filter Comparison");
//     for size in sizes {
//         let sample_data = prepare_sample_data(size).expect("Failed to prepare sample data");
        
//         group.bench_with_input(BenchmarkId::new("WASM", size), &sample_data, |b, data| {
//             b.iter(|| options_processor.process_options(data).unwrap());
//         });
//         group.bench_with_input(BenchmarkId::new("Native", size), &sample_data, |b, data| {
//             b.iter(|| native_filter_options(data));
//         });
//     }
//     group.finish();
// }

// criterion_group!(benches, benchmark_wasm_filter, benchmark_native_filter);
criterion_group!(benches, compare_filters);
criterion_main!(benches);