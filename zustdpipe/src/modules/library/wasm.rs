use super::interface;
use crate::error::error::GeneralError;

use std::collections::HashMap;

use wasmtime::{Engine, Instance, Linker, Module, Store};
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
        name: String,
        _serialized_data: Vec<u8>,
    ) -> Result<Vec<u8>, interface::LibraryInstanceError> {
        // make serialized data available to function
        // call function
        let func_def = &self
            .instance
            .get_func(&mut self.store, &name)
            .expect(format!("`{}` was not an exported function", name).as_str());
        let func_validated = func_def.typed::<(), u64>(&self.store).unwrap();
        let _result = func_validated.call(&mut self.store, ()).unwrap();
        // provide result back
        let result: Vec<u8> = Vec::new();
        Ok(result)
    }
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
        let result_func = result_simple_wat_library.exec_func("simple".to_string(), param);
        Ok(())
    }
}
