use zustdpipe_modules_library::interfaces::process::interface::{Parameters, Result, Process};

pub struct EchoProcess {}

impl Process for EchoProcess {
    fn new() -> Self {
        println!("init");
        return EchoProcess {};
    }

    fn execute(&self, param: Parameters) -> Option<Result> {
        println!("Hallo");
        None
    }
}
