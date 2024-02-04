
use std::path::Path;


use crate::error::error::GeneralError;
use crate::pipeline::interface::ProcessDefinition;
use super::library::wasm::WASMLibraryManager;
use super::interface::{ModuleDefinitionError, ModuleManager, ModuleManagerList, ModuleType, ModulesDefinition};
use super::library::interface::{Library,LibraryManager};

/// Manage modules
impl ModuleManager for ModuleManagerList{
    /// Returns a new Module Manager
    ///
    /// # Examples
    /// ```
    /// use zustdpipe::modules::manager;
    /// let mut modulemgr: manager::ModuleManagerList = manager::ModuleManager::new();
    /// ```
    fn new(modules_definition: &ModulesDefinition) ->Result<ModuleManagerList,ModuleDefinitionError> 

    {
        let module_paths = match get_module_paths(modules_definition) {
            Ok(module_paths) => module_paths,
            Err(error) => return Err(error)
        };
        // load wasm modules
        let wasm_library_manager= match &modules_definition.wasm {
            Some(wasm) => {
                Some(WASMLibraryManager::new())
            },
            None => None
        };
        
        Ok(
            ModuleManagerList {
                wasm_library_manager: wasm_library_manager,
                module_paths: module_paths
            })
    }


    fn get_module_instance(&mut self,process_definition: &ProcessDefinition) -> Result<Box<dyn Library>, ModuleDefinitionError> {
        let mut library_manager =   match &process_definition.module.r#type {
            ModuleType::Wasm => {
                match &mut self.wasm_library_manager {
                    Some(library_manager) => library_manager,
                    None => return Err(ModuleDefinitionError::ModuleTypeNotFound(GeneralError::ErrorMessage(format!("No library manager found for {:?}",&process_definition.module.r#type))))
                }
            }
        };
        let module_full_path= match find_module_in_module_paths(&self.module_paths,&process_definition.module.name) {
            Ok(module_full_path) => module_full_path,
            Err(error) => return Err(error)
        };
        match library_manager.get_instance(&module_full_path) {
            Ok(module_instance) => Ok(module_instance),
            Err(error) => Err(ModuleDefinitionError::ModuleCannotBeInstantiated(error.clone()))
        }
    }

}


 /// Reads the module base path from the modules definition and checks if it exists
fn get_module_paths(modules_definition: &ModulesDefinition) -> Result<Vec<String>,ModuleDefinitionError>{
  
    // check if module paths exist
    match &modules_definition.wasm  {
      Some(wasm) => {
          let mut module_paths=Vec::new();
          for module_path in &wasm.module_path_base {
              let current_module_path = Path::new(module_path);
              if !current_module_path.exists() {
                  return Err(ModuleDefinitionError::ModulePathInvalid(GeneralError::ErrorMessage(format!("Module path \"{}\" does not exist", module_path))));
              }
              module_paths.push(module_path.clone());
          }
          Ok(module_paths)
      },
      None => {
          return Err(ModuleDefinitionError::ModulePathInvalid(GeneralError::ErrorMessage("No path to modules given".to_string())));
      }
  }
}

/// Searches the module paths and returns the first module found matching the name
fn find_module_in_module_paths(module_paths: &Vec<String>,name:&String) -> Result<String,ModuleDefinitionError> {
    for module_path in module_paths {
        let full_path = format!("{}/{}",module_path,name);
        let current_module_full_path = Path::new(&full_path);
        if current_module_full_path.exists()  {
            return Ok(full_path);
        }
    }
    Err(ModuleDefinitionError::ModuleNotFound(GeneralError::ErrorMessage(format!("Could not find module \"{}\" in module paths",name))))
}