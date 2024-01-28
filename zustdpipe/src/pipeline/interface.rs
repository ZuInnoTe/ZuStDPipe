
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::modules::interface::ModuleType;



/// Deifinition of a single pipeline
#[derive(Deserialize, Serialize,Clone)]
pub struct PipelineDefinition {
  pub processs: Vec<HashMap<String,ProcessDefinition>>
}

#[derive(Deserialize, Serialize,Clone,Debug)]
pub struct ProcessModuleRequirements {
  pub name: String, 
  pub r#type: ModuleType
}

/// Deifinition of a single Process
#[derive(Deserialize, Serialize,Clone,Debug)]
pub struct ProcessDefinition {
  pub module: ProcessModuleRequirements, 
  pub parameters: Vec<HashMap<String,String>>
}

