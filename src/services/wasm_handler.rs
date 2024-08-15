use anyhow::{Result, Context};
use wasmtime::*;
use std::path::Path;

pub struct WasmHandler {
    store: Store<()>,
    memory: Memory,
    alloc: TypedFunc<i32, i32>,
    dealloc: TypedFunc<(i32, i32), ()>,
    filter_incidents: TypedFunc<(i32, i32), i32>,
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
        let filter_incidents = instance.get_typed_func(&mut store, "filter_incidents")
            .context("Failed to get filter_incidents function")?;
        let get_result_len = instance.get_typed_func(&mut store, "get_result_len")
            .context("Failed to get get_result_len function")?;

        Ok(Self {
            store,
            memory,
            alloc,
            dealloc,
            filter_incidents,
            get_result_len,
        })
    }

    /// Filter incidents using the WebAssembly module
    pub fn filter_incidents(&mut self, incidents_json: &[u8]) -> Result<Vec<u8>> {
        let incidents_len = incidents_json.len();
        let incidents_ptr = self.alloc.call(&mut self.store, incidents_len as i32)?;
        
        self.memory.write(&mut self.store, incidents_ptr as usize, incidents_json)?;
        
        let result_ptr = self.filter_incidents.call(&mut self.store, (incidents_ptr, incidents_len as i32))?;
        let result_len = self.get_result_len.call(&mut self.store, ())?;
        
        let mut result = vec![0u8; result_len as usize];
        self.memory.read(&self.store, result_ptr as usize, &mut result)?;
        
        self.dealloc.call(&mut self.store, (incidents_ptr, incidents_len as i32))?;
        
        Ok(result)
    }
}