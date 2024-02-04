use zustdpipe::apps::interface::{AppList, AppManager};

use crate::error::CliGeneralError;

/// Validates an application definition file
pub fn validate_application_definition_file(
    application_definition_file: &std::path::PathBuf,
) -> Result<(), CliGeneralError> {
    let f = match std::fs::File::open(application_definition_file) {
        Ok(file) => file,
        Err(_error) => {
            return Err(CliGeneralError::ErrorMessage(format!(
                "Cannot open file: {}",
                application_definition_file.as_path().display().to_string()
            )))
        }
    };
    let mut appmgr: AppList = AppManager::new();
    appmgr.add(f).unwrap();
    println!("Application name: {}", appmgr.get(0).general.name);

    println!("Number of pipelines: {}", &appmgr.get(0).pipelines.len());
    for (name, definition) in &appmgr.get(0).pipelines {
        println!("Pipeline name: {}", &name);
        for process in &definition.process {
            for (process_name, process_definition) in process.iter() {
                println!("Process name: {}", process_name);
            }
        }
    }

    Ok(())
}
