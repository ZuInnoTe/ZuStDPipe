use std::{collections::HashMap, sync::Arc};
use arrow::datatypes::{DataType, Field, Schema};
use uuid::Uuid;

use crate::{
    error::error::GeneralError,
    modules::interface::{ModuleDefinitionError, ModuleManager, ModulesDefinition},
    pipeline::interface::{PipelineDefinition, ProcessDefinition},
};

use super::interface::{JobDefinition, JobList, JobManager, JobRunError, JobValidationError};

impl JobManager for JobList {
    fn run_job(
        &mut self,
        pipeline_definitions: &HashMap<String, PipelineDefinition>,
        job_definition: &JobDefinition,
    ) -> Result<String, JobRunError> {
        // check if we can run the job
        let pipeline_definition = match validate_job(pipeline_definitions, job_definition) {
            Ok(pipeline_definition) => pipeline_definition,
            Err(error) => return Err(JobRunError::JobValidationError(error)),
        };
        let job_id = Uuid::new_v4();
        for process in &pipeline_definition.process {
            for (process_name, process_definition) in process.iter() {
                println!("Process name: {}", process_name);
                let mut module_instance = match self
                    .module_manager_list
                    .get_module_instance(process_definition)
                {
                    Ok(module_instance) => module_instance,
                    Err(error) => return Err(JobRunError::JobModuleDefinitionError(error)),
                };
               // process_definition.parameters
                let serialized_answer=match module_instance.exec_func(vec!(1u8),vec!(1u8)) {
                    Ok(serialized_data) => serialized_data,
                    Err(error) => return Err(JobRunError::JobModuleInstantiationError(error))
                };
            }
        }

      Ok(job_id.to_string())
    }

    // include job metadata
    // include module information
    fn new(modules_definition: &ModulesDefinition) -> Result<Self, ModuleDefinitionError> {
        let module_manager_list = match ModuleManager::new(modules_definition) {
            Ok(module_manager_list) => module_manager_list,
            Err(module_definition_error) => return Err(module_definition_error),
        };
        Ok(JobList {
            map: HashMap::new(),
            module_manager_list: module_manager_list,
        })
    }
}

pub fn validate_job(
    pipeline_definitions: &HashMap<String, PipelineDefinition>,
    job_definition: &JobDefinition,
) -> Result<PipelineDefinition, JobValidationError> {
    let pipeline_definition = match pipeline_definitions.get(&job_definition.pipeline) {
        Some(pipeline_definition) => pipeline_definition,
        None => {
            return Err(JobValidationError::PipelineForJobNotFound(
                GeneralError::ErrorMessage(format!(
                    "Could not find pipeline {} for job",
                    &job_definition.pipeline
                )),
            ))
        }
    };
    Ok(pipeline_definition.clone())
}

/// Converts metadata for a process into arrow format
/// # Arguments
/// * `process_definition` - process definition
fn metadata_to_arrow(process_definition: &ProcessDefinition) -> Vec<u8> {
   // define schema
   let schema = Schema::new(vec![
    Field::new(
        "parameters",
        DataType::Map(Arc::new(
            Field::new("entries",
            DataType::Struct(arrow::datatypes::Fields::from(vec![Field::new(
            "keys",
            DataType::Utf8,
            false,
        ),Field::new(
            "values",
            DataType::Utf8,
            false,
        )])),false)),false),
    false)
]);
//let mut decoder = ReaderBuilder::new(Arc::new(schema)).build_decoder().unwrap();

// add values
/*for param in process_definition.parameters {
    param.
}
let command = StringArray::from(vec!["test"]);

let parameters = StructArray::from(vec![(
    Arc::new(Field::new("filename", DataType::Utf8, false)),
    Arc::new(StringArray::from(vec!["test.txt"])) as ArrayRef,
)]);
// build a record batch
let batch = RecordBatch::try_new(
    Arc::new(schema.clone()),
    vec![Arc::new(command), Arc::new(config)],
)
.unwrap();
// serialize it
let buffer: Vec<u8> = Vec::new();

let mut stream_writer = StreamWriter::try_new(buffer, &schema).unwrap();
stream_writer.write(&batch).unwrap();

let serialized_batch = stream_writer.into_inner().unwrap();
return serialized_batch;
*/
Vec::new()
}