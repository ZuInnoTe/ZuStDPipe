use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::error::error::GeneralError;
use crate::pipeline::interface::PipelineDefinition;

use crate::modules::interface::{ModuleDefinitionError, ModuleManagerList, ModulesDefinition};

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum JobTriggerType {
    Manual,
}

/// Definition of a single job
#[derive(Deserialize, Serialize)]
pub struct JobDefinition {
    pub pipeline: String,
    pub threads: u32,
    pub trigger: JobTriggerType,
}

#[derive(Debug)]
pub enum JobValidationError {
    PipelineForJobNotFound(GeneralError),
}

#[derive(Debug)]
pub enum JobRunError {
    JobStartError(GeneralError),
    JobValidationError(JobValidationError),
    JobModuleDefinitionError(ModuleDefinitionError),
}

/// Properties of a running job
pub struct Job {
    pub name: String,
    pub pipeline_definition: PipelineDefinition,
}

/// The job list contains all running jobs (key is the unique instance id and their Job information) and module managers
pub struct JobList {
    pub map: HashMap<String, Job>,
    pub(crate) module_manager_list: ModuleManagerList,
}

/// The Job manager manages the jobs
pub trait JobManager {
    fn new(modules_definition: &ModulesDefinition) -> Result<Self, ModuleDefinitionError>
    where
        Self: Sized;
    fn run_job(
        &mut self,
        pipeline_definitions: &HashMap<String, PipelineDefinition>,
        job_definition: &JobDefinition,
    ) -> Result<String, JobRunError>;
}
