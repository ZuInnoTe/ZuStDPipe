//! Module Interface (MI) for a Process in ZuStDPipe - this needs to be implemented by any plugable process
// The data is exchanged using Arrow-IPC

use arrow::ipc::RecordBatch;


pub struct Parameters<'a> {
    pub metadata: RecordBatch<'a>,
    pub data: RecordBatch<'a>
}

pub struct Result<'a> {
   pub data: RecordBatch<'a> 
}


/// Implementation of the process
pub trait Process {
    fn new() -> Self;
     fn execute(&self,params: Parameters) -> Option<Result>;
}