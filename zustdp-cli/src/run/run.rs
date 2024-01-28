
use zustdpipe::{apps::interface::{AppList, AppManager}, jobs::interface::{JobList, JobManager}, pipeline};

use crate::error::CliGeneralError;

    pub fn run_job(application_definition_file: &std::path::PathBuf, job_name: &String) -> Result<(),CliGeneralError> {
            let f = match std::fs::File::open(application_definition_file) {
                Ok(file) => file,
                Err(_error) =>   return Err(CliGeneralError::ErrorMessage(format!("Cannot open file: {}",application_definition_file.as_path().display().to_string())))   
            };
            let mut appmgr: AppList = AppManager::new();
            appmgr.add(f).unwrap();
            let app_definition= &appmgr.get(0);
            println!("Application name: {}",&app_definition.general.name);
            println!("Trying to run {}",job_name);
    
            let mut jobmgr: JobList = match JobManager::new(&app_definition.modules) {
                Ok(jobmgr) => jobmgr,
                Err(error) => {
                    return Err(CliGeneralError::ErrorMessage(format!("{:#?}", error)));
                }
            };
                    
                    let job_definitions = &app_definition.jobs;
                    let job_definition = match job_definitions.get(job_name) {
                        Some(job_definition) => job_definition,
                        None =>  return Err(CliGeneralError::ErrorMessage(format!("Job {} not found",&job_name)))
                    };
                    match jobmgr.run_job(&app_definition.pipelines, job_definition) {
                        Ok(job_id) => { 
                            println!("Job id: {}",job_id);
                            Ok(())
                        },
                        Err(error) => Err(CliGeneralError::ErrorMessage(format!("{:#?}", error)))
                    }
                
    }
            
        