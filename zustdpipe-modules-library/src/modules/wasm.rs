//! Provide some generic wasm functions to allow zustdp to request memory from the module to share parameters, (meta-)data and reading feedback

use std::rc::Rc;
use std::cell::Cell;
use std::cell::RefCell;
use std::collections::HashMap;
use std::io::BufReader;
use std::io::Read;
use std::mem::ManuallyDrop;
use std::slice;


use arrow::ipc::reader::StreamReader;
use arrow::ipc::writer::StreamWriter;
use arrow::record_batch::RecordBatch;


// Global variable to keep track of allocated memory
// Note: This is really an execption as allocate by the app to the module should have only for parameters
// Otherwise it would be really bad for performance.
thread_local!(
    static MEMORY_AREAS: RefCell<HashMap<*const u8, (usize, ManuallyDrop<Box<[u8]>>)>> =
        RefCell::new(HashMap::new());
);


enum MemoryAreasReturnCode {
    Success = 0,
    ErrorMemmoryNotAllocated = -1,
}

/// Allocate some memory for the application to write data for the module
/// Note: It is up to the application (and not the WASM module) to provide enough pages, so the module does not run out of memory
/// # Arguments
/// * `size` - size of memory to allocaten
/// returns a pointer to the allocated memory area
#[no_mangle]
pub extern "C" fn zustdp_module_wasm_allocate(size: u32) -> *const u8 {
    // create a Box with empty memory
    let alloc_box = ManuallyDrop::new(vec![0u8; size as usize].into_boxed_slice());
    return allocate(size as usize, alloc_box);
}

/// Deallocates existing memory for the purpose of the application
/// # Arguments
/// * `ptr` - mutuable pointer to the memory to deallocate
/// returns a code if it was successful or not
#[no_mangle]
pub extern "C" fn zustdp_module_wasm_deallocate(ptr: *const u8) -> i32 {
    // check if the ptr exists
    let cell: Cell<Option<(usize, ManuallyDrop<Box<[u8]>>)>> = Cell::new(None);
    MEMORY_AREAS.with(|mem_map| cell.set(mem_map.borrow_mut().remove(&ptr)));
    let memory_area: Option<(usize, ManuallyDrop<Box<[u8]>>)> = cell.into_inner();
    match memory_area {
        Some(x) => ManuallyDrop::into_inner(x.1), // will then be deleted after function returns
        None => return MemoryAreasReturnCode::ErrorMemmoryNotAllocated as i32,
    };
    // return success
    return MemoryAreasReturnCode::Success as i32;
}



/// Allocate some memory for the application to write data for the module
/// Note: It is up to the application (and not the WASM module) to provide enough pages, so the module does not run out of memory
/// This function can also be used internally by the WASM module to return data to the calling application of the module
/// # Arguments
/// * `size` - size of memory to allocaten
/// returns a pointer to the allocated memory area
pub fn allocate(size: usize, alloc_box: ManuallyDrop<Box<[u8]>>) -> *const u8 {
    let result_ptr: *const u8 = alloc_box.as_ptr();
    // save allocated memory to avoid it is cleaned up after function exits
    MEMORY_AREAS.with(|mem_map| mem_map.borrow_mut().insert(result_ptr, (size, alloc_box)));
    return result_ptr;
}

/// Validates if a pointer has been properly allocated in this module
/// # Arguments
/// * `ptr` - pointer
/// returns the size of the allocated memory area. It is 0 if the pointer is invalid
pub fn validate_pointer(ptr: *const u8) -> usize {
    let cell: Cell<usize> = Cell::new(0);
    MEMORY_AREAS.with(|mem_map| match mem_map.borrow().get(&ptr) {
        Some(x) => cell.set(x.0),
        None => cell.set(0),
    });
    return cell.get();
}


/// Converts a raw memory pointer to data in Arrow format to an arrow StreamReader
/// # Arguments
/// * `raw_memory_offset` - pointer to the data
/// * `raw_memory_size` - size of the data
/// returns a StreamReader on the Arrow Data or None if there is an error related to the memory location
pub fn convert_raw_memory_to_arrow(    raw_memory_offset: *mut u32,
    raw_memory_size: u32) -> Option<StreamReader<BufReader<&'static [u8]>>> {
        let expected_size_raw_memory: usize = validate_pointer(raw_memory_offset as *const u8);
        if (expected_size_raw_memory == 0) | (expected_size_raw_memory != expected_size_raw_memory as usize) {
            return None;
        };
        match StreamReader::try_new(unsafe{std::slice::from_raw_parts(raw_memory_offset as *mut u8, raw_memory_size as usize)}, None) {
            Ok(reader) => Some(reader),
            Err(error) => None
        }
}

 /// Create an error message in Arrow format
 pub fn create_error_response_arrow_data() {

 }