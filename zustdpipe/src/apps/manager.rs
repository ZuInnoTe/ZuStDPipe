//! The app manager manages app definitions and their validity.
//! Specific parts of apps are delegated to the corresponding ZuSearch modules (e.g. search, pipelines)

use serde_yaml;
use std::io;

use super::error;
use super::interface::{AppDefinition, AppList, AppManager};

/// Managing the definition of apps in-memory
impl AppManager for AppList {
    /// Returns a new app list to manage app definitions in memory
    ///
    /// # Examples
    /// ```
    /// use zustdpipe::apps::interface;
    /// let mut appmgr: interface::AppList = interface::AppManager::new();
    /// ```
    fn new() -> AppList
    where
        Self: AppManager,
    {
        AppList {
            available_apps: Vec::<AppDefinition>::new(),
        }
    }

    /// Add an app to the list of available apps
    ///
    /// # Arguments
    /// * `rdr` - A reader implementing std::io::Read from where a ZuStdPipe app definition can be read from
    ///
    fn add<R>(&mut self, rdr: R) -> Result<(), error::AppDefinitionError>
    where
        R: io::Read,
    {
        match serde_yaml::from_reader(rdr) {
            Ok(def) => Ok(self.available_apps.push(def)),
            Err(err) => Err(error::AppDefinitionError::Serdeyaml(err)),
        }
    }

    /// Get definition of an app
    ///
    /// # Arguments
    /// * `position` - position of the app in the list
    ///
    fn get(&mut self, position: usize) -> &AppDefinition {
        &self.available_apps[position]
    }

    /// Remove definition of an app
    ///
    /// # Arguments
    /// * `position` - position of the app to remove
    ///
    fn remove(&mut self, position: usize) -> AppDefinition {
        self.available_apps.swap_remove(position)
    }
}

#[cfg(test)]
mod tests {
    const MINIMAL_APP_VALID_STRING: &str =
        "general:\n name: \"ZuStdPipe Example App\"\n app_definition_version: 0\nmodules:\njobs:\npipelines:\n";
    const MINIMAL_APP_INVALID_APP_STRING: &str =
        "special:\n name: \"ZuStdPipe Example App\"\n app_definition_version: 0\nmodules:\njobs:\npipelines:\n";
    const MINIMAL_APP_INVALID_YAML_STRING: &str = "test\ntest";

    #[test]
    // Test a minimal valid app definition
    fn test_new_minimal_valid() {
        // Create in memory String Reader
        use crate::apps::manager::AppManager;
        use std::io::BufReader;
        let str_reader = BufReader::new(MINIMAL_APP_VALID_STRING.as_bytes());
        // Load minimal app definition into memory
        let mut appmgr: crate::apps::manager::AppList = crate::apps::manager::AppManager::new();
        appmgr.add(str_reader).unwrap();
        const EXPECTED_APP_NAME: &str = "ZuStdPipe Example App";
        let actual_app_name: &str = &appmgr.get(0).general.name;
        assert_eq!(
            actual_app_name, EXPECTED_APP_NAME,
            "Application name in definition correct. Actual {}. Expected {}.",
            actual_app_name, EXPECTED_APP_NAME
        );
        const EXPECTED_APP_DEF_VERSION: u32 = 0;
        let actual_app_def_version: u32 = appmgr.get(0).general.app_definition_version;
        assert_eq!(
            actual_app_def_version, EXPECTED_APP_DEF_VERSION,
            "Application definition version correct.  Actual {}. Expected {}.",
            actual_app_def_version, EXPECTED_APP_DEF_VERSION
        );
    }

    #[test]
    // Test an invalid app definition
    fn test_new_minimal_invalid() {
        // Create in memory String Reader
        use super::error;
        use crate::apps::manager::AppManager;
        use std::io::BufReader;
        let str_reader = BufReader::new(MINIMAL_APP_INVALID_APP_STRING.as_bytes());
        // Load minimal app definition into memory
        let mut appmgr: crate::apps::manager::AppList = crate::apps::manager::AppManager::new();
        let result = appmgr.add(str_reader).unwrap_err().to_string();
        let expected =
            "Invalid App definition. Error in Yaml file: missing field `general`".to_string();
        assert_eq!(expected, result);
    }
}
