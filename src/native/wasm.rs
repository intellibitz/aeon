// AEON Wasm Host Substrate
// RULE 11: Native Integration - High-performance reflex execution environment

use std::path::Path;
use wasmer::{Instance, Module, Store};
use wasmer_wasi::{WasiState, Pipe};
use crate::error::{EaiError, EaiResult};

pub struct WasmHost;

// SAFETY: This is a placeholder for the stack probing function required by some Rust targets.
// It is empty because the Wasm runtime handles stack overflows.
#[unsafe(no_mangle)]
pub extern "C" fn __rust_probestack() {}

impl WasmHost {
    /// Executes a distilled reflex from a Wasm file
    pub fn execute_reflex(wasm_path: &Path, arg: &str) -> EaiResult<String> {
        let mut store = Store::default();
        let module = Module::from_file(&store, wasm_path)
            .map_err(|e| EaiError::Process(format!("Failed to load Wasm module: {}", e)))?;

        let output = Pipe::new();
        let mut state_builder = WasiState::new("aeon-reflex");
        state_builder.arg(arg);
        state_builder.stdout(Box::new(output.clone()));

        let wasi_env = state_builder
            .finalize(&mut store)
            .map_err(|e| EaiError::Process(format!("WASI finalize failed: {:?}", e)))?;

        let import_object = wasi_env.import_object(&mut store, &module)
            .map_err(|e| EaiError::Process(format!("WASI import failed: {:?}", e)))?;

        let instance = Instance::new(&mut store, &module, &import_object)
            .map_err(|e| EaiError::Process(format!("Wasm instantiation failed: {}", e)))?;

        let start = instance.exports.get_function("_start")
            .map_err(|_| EaiError::Process("Wasm missing _start entry point".into()))?;

        // This is a blocking call
        start.call(&mut store, &[])
            .map_err(|e| EaiError::Process(format!("Wasm execution failed: {}", e)))?;

        // Capture stdout
        let mut result = String::new();
        use std::io::Read;
        let mut reader = output;
        reader.read_to_string(&mut result).map_err(|e| EaiError::Process(format!("Failed to read Wasm output: {}", e)))?;

        Ok(if result.trim().is_empty() {
             "Wasm execution success (No Output)".to_string()
        } else {
             result.trim().to_string()
        })
    }
}
