use std::cell::Cell;
use std::cell::RefCell;
use std::collections::HashMap;
use std::mem::ManuallyDrop;
use std::sync::Arc;


use arrow::ipc::reader::StreamReader;


use super::echoprocess::EchoProcess;

use zustdpipe_modules_library::interfaces::process::interface::Process;
use zustdpipe_modules_library::modules;

/// This is the raw entry function into any WebAssembly module
/// It takes care that the module parameters and data are passed through the serialization framework and that the answer from the module is provided back as a pointer
pub extern "C" fn zustdp_module_wasm_raw_process_entry(
    meta_data_offset: *mut u32,
    meta_data_size: u32,
    data_offset: *mut u32,
    data_size: u32,
) -> u32 {
    let mut input_arrow_meta_data= match modules::wasm::convert_raw_memory_to_arrow(meta_data_offset,meta_data_size) {
        Some(data) => data,
        None => return 0
    };
    let mut input_arrow_data= match modules::wasm::convert_raw_memory_to_arrow(data_offset,data_size) {
        Some(data) => data,
        None => return 0
    };
    // check parametes
    //for item in input_arrow_meta_data
    // call function
    let process: EchoProcess = Process::new();

    // convert result arrow to raw memory
    return 0;
}
