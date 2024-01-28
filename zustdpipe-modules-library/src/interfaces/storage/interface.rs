//! Module Interface (MI) for Storage in ZuStDPipe - this needs to be implemented by any plugable storage

use super::api::Storage;

/** Module Interface (MI) for storage **/

pub const BLOCKMAGIC: [u8; 4] = [90, 85, 83, 69]; /* ZUSE */

pub enum StorageType {
    Volatile,
    PermanentLocal,
    PermanentRemote,
}

pub struct StorageFile {
    // of blocks
}

pub struct StorageIdData {
    id: Vec<u8>, // tbc how to store ids
    position: u32,
    size: u32,
}

pub struct StorageBlockMetadata {
    version: u32,
    size_processed: u64,
    size_original: u64,
    ids: StorageIdData,
    writer_identifier: String,
    applied_modules: Vec<String>, // applied modules, e.g. compression, encryption, datatype
}

pub struct StorageBlock {
    magic_start: [u8; 4],
    block_data: Vec<u8>,
    lookup_data: Vec<u8>, // bloomfilter, min/max index, dictionary... just to see if it is potentially in there
    metadata: StorageBlockMetadata,
    magic_end: [u8; 4],
}

pub trait ModuleStorage {
    fn add_block(&self); // add block
}
