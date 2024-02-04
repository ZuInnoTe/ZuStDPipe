use std::{collections::HashMap};
use uuid::Uuid;

use crate::{error::error::GeneralError, modules::interface::{ModuleDefinitionError, ModuleManager, ModulesDefinition}, pipeline::interface::PipelineDefinition};

use super::interface::{JobDefinition, JobList, JobManager, JobRunError, JobValidationError};



impl JobManager for JobList {
    fn run_job(&mut self, pipeline_definitions: &HashMap<String,PipelineDefinition>,job_definition: &JobDefinition) -> Result<String,JobRunError> {
        // check if we can run the job
        let pipeline_definition=match validate_job(pipeline_definitions,job_definition) {
            Ok(pipeline_definition) => (pipeline_definition),
            Err(error) => return Err(JobRunError::JobValidationError(error))
        };
        let job_id = Uuid::new_v4();
        for process in &pipeline_definition.processs {
            for (process_name,process_definition) in process.iter() {
                println!("Process name: {}",process_name);
                let mut module_instance=match self.module_manager_list.get_module_instance(process_definition) {
                    Ok(module_instance) => module_instance,
                    Err(error) => return Err(JobRunError::JobModuleDefinitionError(error))
                };
                //module_instance.exec_func("test_func".to_string(),Vec::new());

            }
        }
  
        return Ok(job_id.to_string())
    }

    // include job metadata
    // include module information
    fn new (modules_definition: &ModulesDefinition) -> Result<Self,ModuleDefinitionError> {
       
        let module_manager_list= match ModuleManager::new(modules_definition) {
            Ok(module_manager_list) => module_manager_list,
            Err(module_definition_error) => return Err(module_definition_error)
        };
        Ok(
        JobList {
            map : HashMap::new(),
            module_manager_list:module_manager_list
        })
    }
}

pub fn validate_job(pipeline_definitions: &HashMap<String,PipelineDefinition>,job_definition: &JobDefinition) -> Result<PipelineDefinition,JobValidationError> {
    let pipeline_definition= match pipeline_definitions.get(&job_definition.pipeline) {
        Some(pipeline_definition) => pipeline_definition,
        None =>   return Err(JobValidationError::PipelineForJobNotFound(GeneralError::ErrorMessage(format!("Could not find pipeline {} for job",&job_definition.pipeline))))
    };
    Ok(pipeline_definition.clone())
}