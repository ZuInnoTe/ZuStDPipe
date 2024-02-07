use super::interface;
use crate::error::error::GeneralError;

use std::collections::HashMap;

use wasmtime::{AsContextMut, Engine, Instance, Linker, Module, Store};
use wasmtime_wasi::{sync::WasiCtxBuilder, WasiCtx};

struct WASMState {
    wasi: WasiCtx,
}

pub struct WASMLibrary {
    path: String,
    instance: Instance,
    store: Store<WASMState>,
}

impl interface::Library for WASMLibrary {
    fn exec_func(
        &mut self,
        serialized_metadata: Vec<u8>,
        serialized_data: Vec<u8>,
    ) -> Result<Vec<u8>, interface::LibraryInstanceError> {
        // make serialized data available to function
        // call function
        let func_def = &self
            .instance
            .get_func(&mut self.store, "zustdp_module_wasm_raw_process_entry")
            .expect(
                format!(
                    "`{}` was not an exported function",
                    "zustdp_module_wasm_raw_process_entry"
                )
                .as_str(),
            );
        let func_validated = func_def
            .typed::<(u32, u32, u32, u32), u32>(&self.store)
            .unwrap();
        // prepare handing Arrow data
        let serialized_metadata_size = serialized_metadata.len();
        let serialized_data_size = serialized_data.len();

        // instantiate memory
        let memory = match self.instance.get_memory(&mut self.store, "memory") {
            Some(memory) => memory,
            None => {
                return Err(interface::LibraryInstanceError::InstantiationError(
                    GeneralError::ErrorMessage(format!("Cannot instantiate module memory")),
                ))
            }
        };
        // allocate some memory within the WASM module for metadata
        let offset_meta_data: u32 = wrapper_wasm_allocate(
            self.instance,
            &mut self.store,
            serialized_metadata_size as u32,
        )
        .unwrap() as u32;
        memory
            .write(
                &mut self.store,
                offset_meta_data.try_into().unwrap(),
                serialized_metadata.as_slice(),
            )
            .unwrap();
        // allocate some memory within the WASM module for data
        let offset_data: u32 =
            wrapper_wasm_allocate(self.instance, &mut self.store, serialized_data_size as u32)
                .unwrap() as u32;
        memory
            .write(
                &mut self.store,
                offset_data.try_into().unwrap(),
                serialized_data.as_slice(),
            )
            .unwrap();
        // call function answer
        let result_offset = match func_validated.call(
            &mut self.store,
            (
                offset_meta_data,
                serialized_metadata_size as u32,
                offset_data,
                serialized_data_size as u32,
            ),
        ) {
            Ok(result) => result,
            Err(_err) => {
                return Err(interface::LibraryInstanceError::InstantiationError(
                    GeneralError::ErrorMessage(format!("Cannot instantiate function")),
                ))
            }
        };
        // deallocate shared WASM Module memory
        let dealloc_metadata_code: i32 = wrapper_wasm_deallocate(
            self.instance,
            &mut self.store,
            offset_meta_data as *const u8,
        )
        .unwrap();
        if dealloc_metadata_code != 0 {
            println!("Error: Could not deallocate shared WASM module memory for meta data");
        }
        let dealloc_data_code: i32 =
            wrapper_wasm_deallocate(self.instance, &mut self.store, offset_data as *const u8)
                .unwrap();
        if dealloc_data_code != 0 {
            println!("Error: Could not deallocate shared WASM module memory for data");
        }
        if result_offset == 0 {
            return Err(interface::LibraryInstanceError::InstantiationError(
                GeneralError::ErrorMessage(format!("Invalid return code.")),
            ));
        } else {
            let mut result_offset_position = result_offset;
            // read answer from memory: these are two values: offset of the processed data and size of the processed data in Arrow IPC format
            // read metadata (offset and size of the Arrow IPC data)
            // note: WebAssembly is by default 32 bit
            let mut ptr_buffer = [0u8; (u32::BITS / 8) as usize];
            let mut len_buffer = [0u8; (u32::BITS / 8) as usize];
            match memory.read(
                &self.store,
                result_offset_position.try_into().unwrap(),
                &mut ptr_buffer,
            ) {
                Ok(()) => (),
                Err(_err) => {
                    return Err(interface::LibraryInstanceError::InstantiationError(
                        GeneralError::ErrorMessage(format!(
                            "Cannot read metadata pointer from module memory."
                        )),
                    ))
                }
            };
            result_offset_position += (u32::BITS / 8) as u32;
            match memory.read(
                &self.store,
                result_offset_position.try_into().unwrap(),
                &mut len_buffer,
            ) {
                Ok(()) => (),
                Err(_err) => {
                    return Err(interface::LibraryInstanceError::InstantiationError(
                        GeneralError::ErrorMessage(format!(
                            "Cannot read metadata pointer from module memory."
                        )),
                    ))
                }
            };
            let result_ptr = u32::from_le_bytes(ptr_buffer);
            let result_len = u32::from_le_bytes(len_buffer);
            // read the Arrow IPC data
            let mut result_arrow_ipc: Vec<u8> = vec![0; result_len as usize];
            let mut result_arrow_ipc_buffer = result_arrow_ipc.as_mut_slice();
            match memory.read(
                &self.store,
                result_ptr.try_into().unwrap(),
                &mut result_arrow_ipc_buffer,
            ) {
                Ok(()) => (),
                Err(_err) => {
                    return Err(interface::LibraryInstanceError::InstantiationError(
                        GeneralError::ErrorMessage(format!(
                            "Cannot read resuts from module memory."
                        )),
                    ))
                }
            };
            let dealloc_return_meta_code: i32 =
                wrapper_wasm_deallocate(self.instance, &mut self.store, result_offset as *const u8)
                    .unwrap();
            if dealloc_return_meta_code != 0 {
                println!(
                    "Error: Could not deallocate shared WASM module memory for return metadata"
                );
            }
            let dealloc_return_data_code: i32 =
                wrapper_wasm_deallocate(self.instance, &mut self.store, result_ptr as *const u8)
                    .unwrap();
            if dealloc_return_data_code != 0 {
                println!("Error: Could not deallocate shared WASM module memory for return data");
            }

            return Ok(result_arrow_ipc);
        }

        Err(interface::LibraryInstanceError::InstantiationError(
            GeneralError::ErrorMessage(format!("Cannot execute function from module")),
        ))
    }
}

/// Wrapper around the allocate function of the WASM module to allocate shared WASM memory. Allocate some memory for the application to write data for the module
/// Note: It is up to the application (and not the WASM module) to provide enough pages, so the module does not run out of memory
/// # Arguments
/// * `size` - size of memory to allocate
/// returns a pointer to the allocated memory area
fn wrapper_wasm_allocate(
    instance: Instance,
    mut store: impl AsContextMut<Data = WASMState>,
    size: u32,
) -> anyhow::Result<*const u8> {
    // Load function an instantiate it

    // get the function
    let func_def = instance
        .get_func(&mut store, "zustdp_module_wasm_allocate")
        .expect("`wasm_allocate` was not an exported function");
    // validate that it corresponds to the parameters and return types we need
    let func_validated = func_def.typed::<u32, u32>(&store)?;
    // call function
    let result = func_validated.call(&mut store, size)?;
    Ok(result as *const u8)
}

///  Wrapper around the deallocate function of the WASM module to deallocate shared WASM memory. Deallocates existing memory for the purpose of the application
/// # Arguments
/// * `ptr` - mutuable pointer to the memory to deallocate
/// returns a code if it was successful or not
fn wrapper_wasm_deallocate(
    instance: Instance,
    mut store: impl AsContextMut<Data = WASMState>,
    ptr: *const u8,
) -> anyhow::Result<i32> {
    // get the function
    let func_def = instance
        .get_func(&mut store, "zustdp_module_wasm_deallocate")
        .expect("`wasm_deallocate` was not an exported function");
    // validate that it corresponds to the parameters and return types we need
    let func_validated = func_def.typed::<u32, i32>(&store)?;
    // call function
    let result = func_validated.call(&mut store, ptr as u32)?;
    Ok(result)
}

pub struct WASMLibraryManager {
    loaded_modules: HashMap<String, Module>,
    engine: Engine,
}

impl interface::LibraryManager<WASMLibrary> for WASMLibraryManager {
    fn new() -> WASMLibraryManager
    where
        Self: interface::LibraryManager<WASMLibrary>,
    {
        WASMLibraryManager {
            loaded_modules: HashMap::<String, Module>::new(),
            engine: Engine::default(),
        }
    }
    fn get_instance(
        &mut self,
        path: &String,
    ) -> Result<Box<WASMLibrary>, interface::LibraryDefinitionError> {
        // check if WASM module has already been loaded
        let module = match self.loaded_modules.get(path) {
            Some(x) => Some(x),
            None => {
                match Module::from_file(
                    // if no => load it
                    &self.engine,
                    &path,
                ) {
                    Ok(x) => {
                        self.loaded_modules.insert(path.clone(), x);
                        self.loaded_modules.get(path)
                    }
                    Err(err) => {
                        return Err(interface::LibraryDefinitionError::ModuleSpecificError(
                            GeneralError::ErrorMessage(format!(
                                "WASM Library Manager. Error during loading module: {}",
                                err.to_string()
                            )),
                        ))
                    }
                }
            }
        }
        .unwrap();
        // lets create an instance from it
        // Link WASI into the module
        let mut linker = Linker::new(&self.engine);
        match wasmtime_wasi::add_to_linker(&mut linker, |state: &mut WASMState| &mut state.wasi) {
            Ok(_x) => (),
            Err(err) => {
                return Err(interface::LibraryDefinitionError::ModuleSpecificError(
                    GeneralError::ErrorMessage(format!(
                        "WASM Library Manager. Error adding WASI to Linker: {}",
                        err.to_string()
                    )),
                ))
            }
        };
        let wasi = match WasiCtxBuilder::new().inherit_stdio().inherit_args() {
            Ok(x) => x.build(),
            Err(err) => {
                return Err(interface::LibraryDefinitionError::ModuleSpecificError(
                    GeneralError::ErrorMessage(format!(
                        "WASM Library Manager. Error creating WASI context: {}",
                        err.to_string()
                    )),
                ))
            }
        };
        let mut store = Store::new(&self.engine, WASMState { wasi: wasi });
        match linker.module(&mut store, "", &module) {
            Ok(_x) => (),
            Err(err) => {
                return Err(interface::LibraryDefinitionError::ModuleSpecificError(
                    GeneralError::ErrorMessage(format!(
                        "WASM Library Manager. Error linking WASI context: {}",
                        err.to_string()
                    )),
                ))
            }
        };
        let instance: Instance = linker.instantiate(&mut store, &module).unwrap();
        let result: Box<WASMLibrary> = Box::new(WASMLibrary {
            path: path.clone(),
            instance: instance,
            store: store,
        });
        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use crate::modules::library::interface;

    const SIMPLE_WAT_PATH: &str = "tests/data/modules/library/wasm/simple.wat";

    #[test]
    // Test a minimal valid WASM app
    fn test_new_minimal_valid_wasm() -> Result<(), interface::LibraryDefinitionError> {
        use crate::modules::library::interface::LibraryManager;
        use crate::modules::library::wasm::WASMLibraryManager;
        // Create a new library manager
        let mut libmgr: WASMLibraryManager =
            crate::modules::library::wasm::WASMLibraryManager::new();
        // try to load a test
        let result_simple_wat_library = libmgr.get_instance(&SIMPLE_WAT_PATH.to_string());
        assert_eq!(result_simple_wat_library?.path, SIMPLE_WAT_PATH);
        Ok(())
    }

    #[test]
    // Test calling a minimal function
    fn test_new_minimal_call() -> Result<(), interface::LibraryDefinitionError> {
        use crate::modules::library::interface::Library;
        use crate::modules::library::interface::LibraryManager;
        use crate::modules::library::wasm::WASMLibraryManager;
        // Create a new library manager
        let mut libmgr: WASMLibraryManager =
            crate::modules::library::wasm::WASMLibraryManager::new();
        // try to load a test
        let result_simple_wat_library = &mut *libmgr.get_instance(&SIMPLE_WAT_PATH.to_string())?;
        assert_eq!(&result_simple_wat_library.path, SIMPLE_WAT_PATH);
        // try to call function
        let param: Vec<u8> = Vec::new();
        let result_func = result_simple_wat_library.exec_func(param);
        Ok(())
    }
}
