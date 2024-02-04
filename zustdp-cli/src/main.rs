use std::{fmt, process::ExitCode};

///! ZuSearch Command Line Interface (CLI)
use structopt::StructOpt;

use crate::{run::run::run_job, validate::validate::validate_application_definition_file};

pub mod error;
pub mod run;
pub mod validate;

#[derive(StructOpt, Debug)]
enum Command {
    Validate,
    Run(RunParameter),
}

#[derive(StructOpt, Debug)]
struct RunParameter {
    #[structopt(short = "-n", long = "--name")]
    name: String,
}

/// Command line arguments

#[derive(StructOpt)]
#[structopt(name = "zustdp-cli", about = "Command line options for zustdp-cli")]
struct Arguments {
    // The path of the config file to read
    #[structopt(long = "application-definition-file", parse(from_os_str))]
    application_definition_file: std::path::PathBuf,
    #[structopt(subcommand)]
    command: Command,
}

fn main() -> ExitCode {
    let args = Arguments::from_args();
    println!("Welcome to zustdp-cli!");
    println!("Checking library zustdpipe...");
    // print library version
    println!("ZuStDPipe Library version: {}", zustdpipe::version());

    println!("Operation: {:?}", &args.command);
    match &args.command {
        Command::Validate => {
            println!(
                "Loading app definition: {:?}",
                &args.application_definition_file
            );
            match validate_application_definition_file(&args.application_definition_file) {
                Ok(()) => ExitCode::from(0),
                Err(error) => {
                    println!("{:#?}", error);
                    ExitCode::from(1)
                }
            }
        }
        Command::Run(parameter) => {
            match run_job(&args.application_definition_file, &parameter.name) {
                Ok(()) => ExitCode::from(0),
                Err(error) => {
                    println!("{:#?}", error);
                    ExitCode::from(1)
                }
            }
        }
    }
}
