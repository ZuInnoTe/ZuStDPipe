use serde::{Deserialize, Serialize};

/// Definition of a library
#[derive(Deserialize, Serialize)]
pub struct LibraryDefinition {
    pub path: String,
}

/// Error in case of an issue with a Library Definition
#[derive(Debug)]
pub enum LibraryDefinitionError {
    ModuleSpecificError(String),
}

#[derive(Debug)]
pub enum AppDefinitionError {
    Serdeyaml(serde_yaml::Error),
}

/// Error in case of an issue with a Library Instance
#[derive(Debug, Clone)]
pub struct LibraryInstanceError;

/// Represents a dynamically loaded library
pub trait Library {
    fn exec_func(
        &mut self,
        name: String,
        serialized_data: &Vec<u8>,
    ) -> Result<Vec<u8>, LibraryInstanceError>;
}

/// Manage dynamically loaded libraries
pub trait LibraryManager<T: Library> {
    fn new() -> Self;
    fn get_instance(&mut self, path: String) -> Result<Box<T>, LibraryDefinitionError>;
}
