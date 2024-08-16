use anyhow::{Result, Context};
use std::path::Path;
use crate::models::types::HistoricalOption;
use crate::services::wasm_handler::WasmHandler;

pub struct OptionsProcessor {
    wasm_handler: WasmHandler,
}

impl OptionsProcessor {
    pub fn new(wasm_file: &Path) -> Result<Self> {
        let wasm_handler = WasmHandler::new(wasm_file)
            .context("Failed to initialize WasmHandler")?;
        Ok(Self { wasm_handler })
    }

    pub fn process_options(&mut self, options: &[HistoricalOption]) -> Result<Vec<HistoricalOption>> {
        let options_json = serde_json::to_vec(options)
            .context("Failed to serialize options")?;
        let filtered_json = self.wasm_handler.filter_historical_options(&options_json)
            .context("Failed to filter options using WebAssembly")?;
        serde_json::from_slice(&filtered_json)
            .context("Failed to deserialize filtered options")
    }

    pub fn print_filtered_options(&self, options: &[HistoricalOption]) {
        println!("Filtered Historical Options received!");
        // for option in options {
        //     println!("{:#?}", option);
        // }
    }
}