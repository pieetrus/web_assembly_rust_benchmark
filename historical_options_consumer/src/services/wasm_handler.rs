use anyhow::{Result, Context};
use wasmtime::*;
use std::path::Path;

pub struct WasmHandler {
    store: Store<()>,
    memory: Memory,
    alloc: TypedFunc<i32, i32>,
    dealloc: TypedFunc<(i32, i32), ()>,
    filter_historical_options: TypedFunc<(i32, i32), i32>,
    get_result_len: TypedFunc<(), i32>,
}

impl WasmHandler {
    /// Initialize a new WasmHandler with the given WebAssembly file
    pub fn new(wasm_file: &Path) -> Result<Self> {
        let engine = Engine::default();
        
        let module = Module::from_file(&engine, wasm_file)
            .context("Failed to load WebAssembly module")?;
        let mut store = Store::new(&engine, ());
        let instance = Instance::new(&mut store, &module, &[])
            .context("Failed to create WebAssembly instance")?;

        let memory = instance.get_memory(&mut store, "memory")
            .context("Failed to get memory")?;
        let alloc = instance.get_typed_func(&mut store, "alloc")
            .context("Failed to get alloc function")?;
        let dealloc = instance.get_typed_func(&mut store, "dealloc")
            .context("Failed to get dealloc function")?;
        let filter_historical_options = instance.get_typed_func(&mut store, "filter_historical_options")
            .context("Failed to get filter_historical_options function")?;
        let get_result_len = instance.get_typed_func(&mut store, "get_result_len")
            .context("Failed to get get_result_len function")?;

        Ok(Self {
            store,
            memory,
            alloc,
            dealloc,
            filter_historical_options,
            get_result_len,
        })
    }

    /// Filter historical options using the WebAssembly module
    pub fn filter_historical_options(&mut self, options_json: &[u8]) -> Result<Vec<u8>> {
        let options_len = options_json.len();
        let options_ptr = self.alloc.call(&mut self.store, options_len as i32)?;
        self.memory.write(&mut self.store, options_ptr as usize, options_json)?;

        let result_ptr = self.filter_historical_options.call(&mut self.store, (options_ptr, options_len as i32))?;
        let result_len = self.get_result_len.call(&mut self.store, ())?;

        let mut result = vec![0u8; result_len as usize];
        self.memory.read(&self.store, result_ptr as usize, &mut result)?;

        self.dealloc.call(&mut self.store, (options_ptr, options_len as i32))?;

        Ok(result)
    }
}