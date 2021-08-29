//! The app manager manages app definitions and their validity. 
//! Specific parts of apps are delegated to the corresponding ZuSearch modules (e.g. search, pipelines)


use std::fmt;
use std::io;
use serde::{Deserialize, Serialize};
use serde_yaml;

/// Error in case the definition of an App is invalid
#[derive(Debug, Clone)]
pub struct AppDefinitionError;

/// List of apps currently in memory
pub struct AppList {
   pub(super) available_apps:  Vec<AppDefinition>
}

/// Manage apps and their definition. It does NOT include executing pipelines or running queries. This is is done by the JobManager
pub trait AppManager {
   fn new() -> Self;
   fn add_app<R>(&mut self, rdr: R) -> Result<(),AppDefinitionError> where R: io::Read;
   fn get_app(&mut self, position: usize) -> &AppDefinition; 
   // fn export_pipeline(&mut self, position: usize) -> String;
  // fn update_pipeline(position: u32, pipeline_str: String);
  // fn update_pipeline(position: u32, pipeline_def: PipelineDefinition) -> Result<(), PipelineDefinitionError>;
   //fn remove_pipeline(position: u32);
}

/// Display a proper error message for invalid pipelone
impl fmt::Display for AppDefinitionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Invalid app definition")
    }
}

/// Definition of an app
#[derive(Deserialize,Serialize)]
pub struct AppDefinition {
    pub general: AppDefinitionGeneral
}


/// General properties of an app in its definition
#[derive(Deserialize,Serialize)]
pub struct AppDefinitionGeneral {
   pub name: String,
   pub zusearch_app_definition_version: u32
}

/// Managing the definition of apps in-memory
impl AppManager for AppList {
   /// Returns a new app list to manage app definitions in memory
   /// 
   /// # Examples
   /// ```
   /// use zusearch::app::manager;
   /// let mut appmgr: manager::AppList = manager::AppManager::new();
   /// ```
    fn new() -> AppList where Self: AppManager {
       AppList {
          available_apps : Vec::<AppDefinition>::new()
       }
    }

   /// Add an app to the list of available apps
   ///
   /// # Arguments
   /// * `rdr` - A reader implementing std::io::Read from where a ZuSearch app definition can be read from
   /// 
   fn add_app<R>(&mut self, rdr: R) -> Result<(),AppDefinitionError> where R: io::Read{
      let def: AppDefinition = serde_yaml::from_reader(rdr).unwrap();
      Ok(self.available_apps.push(def))
 
    }
 
    fn get_app(&mut self, position: usize) -> &AppDefinition {
       &self.available_apps[position]
    }
    
 }
 
#[cfg(test)]
mod tests {
   const MINIMAL_APP_VALID_STRING: &str = "general:\n name: \"ZuSearch Example App\"\n zusearch_app_definition_version: 0\n";

    #[test]
    fn test_new_minimal_valid() {
         // Create in memory String Reader
         use std::io::BufReader;
         use crate::app::manager::AppManager;
         let str_reader = BufReader::new(MINIMAL_APP_VALID_STRING.as_bytes());
         // Load minimal app definition into memory
         let mut appmgr: crate::app::manager::AppList = crate::app::manager::AppManager::new();
         appmgr.add_app(str_reader).unwrap();
         const EXPECTED_APP_NAME: &str = "ZuSearch Example App";
         let actual_app_name: &str = &appmgr.get_app(0).general.name;
         assert_eq!(actual_app_name,EXPECTED_APP_NAME,"Application name in definition correct. Actual {}. Expected {}.",actual_app_name,EXPECTED_APP_NAME);
         const EXPECTED_APP_DEF_VERSION: u32 = 0;
         let actual_app_def_version: u32 = appmgr.get_app(0).general.zusearch_app_definition_version;
         assert_eq!(actual_app_def_version,EXPECTED_APP_DEF_VERSION,"Application definition version correct.  Actual {}. Expected {}.",actual_app_def_version,EXPECTED_APP_DEF_VERSION);
    } 

    #[test]
    fn test_new_minimal_invalid() {
      // Create in memory String Reader
      use std::io::BufReader;
      use crate::app::manager::AppManager;
      let str_reader = BufReader::new(MINIMAL_APP_VALID_STRING.as_bytes());
      // Load minimal app definition into memory
      let mut appmgr: crate::app::manager::AppList = crate::app::manager::AppManager::new();
      appmgr.add_app(str_reader).unwrap();
      const EXPECTED_APP_NAME: &str = "ZuSearch Example App";
      let actual_app_name: &str = &appmgr.get_app(0).general.name;
      assert_eq!(actual_app_name,EXPECTED_APP_NAME,"Application name in definition correct. Actual {}. Expected {}.",actual_app_name,EXPECTED_APP_NAME);
      const EXPECTED_APP_DEF_VERSION: u32 = 0;
      let actual_app_def_version: u32 = appmgr.get_app(0).general.zusearch_app_definition_version;
      assert_eq!(actual_app_def_version,EXPECTED_APP_DEF_VERSION,"Application definition version correct.  Actual {}. Expected {}.",actual_app_def_version,EXPECTED_APP_DEF_VERSION);
 } 

}
