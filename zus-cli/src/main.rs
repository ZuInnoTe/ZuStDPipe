///! ZuSearch Command Line Interface (CLI)
use structopt::StructOpt;
use zusearch::app;
use zusearch::app::manager::AppManager;

#[derive(StructOpt, Debug)]
enum Command {
    Validate,
    Trigger,
    Search,
}

/// Command line arguments

#[derive(StructOpt)]
#[structopt(name = "zus-cli", about = "Command line options for zus-cli")]
struct Arguments {
    // The path of the config file to read
    #[structopt(long = "app-file", parse(from_os_str))]
    appfile: std::path::PathBuf,
    #[structopt(subcommand)]
    command: Command,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Arguments::from_args();
    println!("Welcome to zus-cli!");
    println!("Checking library zusearch...");
    // print library version
    println!("ZuSearch Library version: {}", zusearch::version());

    println!("Loading app definition");
    println!("{:?}", &args.appfile);
    println!("Operation {:?}", &args.command);

    let mut appmgr: app::manager::AppList = app::manager::AppManager::new();
    let f = std::fs::File::open(&args.appfile).expect(&format!(
        "Cannot open file: {}",
        &args.appfile.as_path().display().to_string()
    ));
    appmgr.add(f).unwrap();

    println!("{}", appmgr.get(0).general.name);

    Ok(())
}
