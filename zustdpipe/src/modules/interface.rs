use crate::error::error::GeneralError;
use crate::pipeline::interface::ProcessDefinition;
use serde::{Deserialize, Serialize};

use super::library::interface::{Library, LibraryDefinitionError};
use super::library::wasm::WASMLibraryManager;

#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(rename_all = "lowercase")]
pub enum ModuleType {
    Wasm,
}

#[derive(Debug)]
pub enum ModuleDefinitionError {
    ModulePathInvalid(GeneralError),
    ModuleNotFound(GeneralError),
    ModuleTypeNotFound(GeneralError),
    ModuleCannotBeInstantiated(LibraryDefinitionError),
}

/// Deifinition of a single pipeline
#[derive(Deserialize, Serialize)]
pub struct ModulesDefinition {
    pub wasm: Option<WasmModulesDefinition>,
}

#[derive(Deserialize, Serialize)]
pub struct WasmModulesDefinition {
    pub module_path_base: Vec<String>,
}

pub struct ModuleManagerList {
    pub wasm_library_manager: Option<WASMLibraryManager>,
    pub(crate) module_paths: Vec<String>,
}

/// Creates a new module manager
pub trait ModuleManager {
    fn new(modules_definition: &ModulesDefinition) -> Result<Self, ModuleDefinitionError>
    where
        Self: Sized;
    fn get_module_instance(
        &mut self,
        process_definition: &ProcessDefinition,
    ) -> Result<Box<dyn Library>, ModuleDefinitionError>;
}
