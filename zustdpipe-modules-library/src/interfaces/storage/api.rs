//! Application Programming Interface (API) for Storage in ZuStDPipe - this is provided by ZuStDPipe for applications to access a registered storage

pub trait StorageManager<S: Storage> {
    fn add(&self);
    // fn get(&self);
    // fn remove(&self);
    //fn list(&self) -> Vec<S>;
}

pub trait Storage {
    // Index Storage
    // Raw Storage
    fn read_block_header(&self); // read block header
    fn query_attributes_by_id(&self); // get all attributes acc
    fn query_ids_by_attribute(&self); //
                                      // read block => with  optional function on lookup data (e.g. bloom filter etc.) for highe  performancer, lookup data may contain a bloomfilter if
                                      // data is in there
                                      // write block
                                      // force flush writing / closure of block
}
