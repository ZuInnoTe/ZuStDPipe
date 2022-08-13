//! The module manager manages dynamically loaded modules
use std::fmt;

use super::library;

/// List of modules currently in memory
pub struct ModuleList {
    modules_cache: Vec<String>,
    modules_extract: Vec<String>,
    modules_filter: Vec<String>,
    modules_index: Vec<String>,
    modules_remotecall: Vec<String>,
    modules_search: Vec<String>,
    modules_storage: Vec<String>,
    modules_transformer: Vec<String>,
    loaded_libraries: Vec<String>,
}

/// Error in case the definition of a module is invalid
#[derive(Debug, Clone)]
pub struct ModuleDefinitionError;

/// Display a proper error message for invalid module
impl fmt::Display for ModuleDefinitionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Invalid module definition")
    }
}

/// Manage modules and their definition.
pub trait ModuleManager {
    fn new() -> Self;
    fn add_module<R>(
        &mut self,
        kind: super::interface::ModuleKind,
        path: String,
    ) -> Result<(), ModuleDefinitionError>;
    fn get_module(&mut self, kind: super::interface::ModuleKind, position: usize) -> &String;
    // fn export_pipeline(&mut self, position: usize) -> String;
    // fn update_pipeline(position: u32, pipeline_str: String);
    // fn update_pipeline(position: u32, pipeline_def: PipelineDefinition) -> Result<(), PipelineDefinitionError>;
    //fn remove_pipeline(position: u32);
}

/// Managing of modules in-memory
impl ModuleManager for ModuleList {
    /// Returns a new module list to manage module definitions in memory
    ///
    /// # Examples
    /// ```
    /// use zusearch::modules::manager;
    /// let mut modulemgr: manager::ModuleList = manager::ModuleManager::new();
    /// ```
    fn new() -> ModuleList
    where
        Self: ModuleManager,
    {
        ModuleList {
            modules_cache: Vec::<String>::new(),
            modules_extract: Vec::<String>::new(),
            modules_filter: Vec::<String>::new(),
            modules_index: Vec::<String>::new(),
            modules_remotecall: Vec::<String>::new(),
            modules_search: Vec::<String>::new(),
            modules_storage: Vec::<String>::new(),
            modules_transformer: Vec::<String>::new(),
            loaded_libraries: Vec::<String>::new(),
        }
    }

    fn add_module<R>(
        &mut self,
        kind: super::interface::ModuleKind,
        path: String,
    ) -> Result<(), ModuleDefinitionError> {
        println!("this is a test");
        Ok(())
    }

    fn get_module(&mut self, kind: super::interface::ModuleKind, position: usize) -> &String {
        &self.modules_cache[position]
    }
}

#[cfg(test)]
mod tests {}
