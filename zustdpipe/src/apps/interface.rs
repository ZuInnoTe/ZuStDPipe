use super::error;

use crate::jobs::interface::JobDefinition;
use crate::modules::interface::ModulesDefinition;
use crate::pipeline::interface::PipelineDefinition;
use serde::{Deserialize, Serialize};
use std::io;

use std::collections::HashMap;

/// List of apps currently in memory
pub struct AppList {
    pub(super) available_apps: Vec<AppDefinition>,
}

/// Manage apps and their definition. It does NOT include executing pipelines or running queries. This is is done by the JobManager
pub trait AppManager {
    fn new() -> Self;
    fn add<R>(&mut self, rdr: R) -> Result<(), error::AppDefinitionError>
    where
        R: io::Read;
    fn get(&mut self, position: usize) -> &AppDefinition;
    fn remove(&mut self, position: usize) -> AppDefinition;
}

/// Definition of an app
#[derive(Deserialize, Serialize)]
pub struct AppDefinition {
    pub general: AppDefinitionGeneral,
    pub modules: ModulesDefinition,
    pub jobs: HashMap<String, JobDefinition>,
    pub pipelines: HashMap<String, PipelineDefinition>,
}

/// General properties of an app in its definition
#[derive(Deserialize, Serialize)]
pub struct AppDefinitionGeneral {
    pub name: String,
    pub app_definition_version: u32,
}

/// Validates a definition
pub trait DefinitionValidator {
    fn validate() -> Result<(), String>;
}
