use crate::crypto::hash::{Hashable, H256};
use bincode::{deserialize, serialize};
use rocksdb::{ColumnFamily, Options};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Mutex;

/// Column family names
pub const PROPOSER_NODE_DATA_CF: &str = "PND";
pub const VOTER_NODE_DATA_CF: &str = "VND";
pub const LEDGER_CF: &str = "LED";
pub const PROP_TREE_LEADER_VEC_CF: &str = "PTLV";
pub const PROP_TREE_PROP_BLOCKS_CF: &str = "PTPB";

/// Database that stores blockchain.
pub struct BlockChainDatabase {
    /// The underlying RocksDB handle.
    pub handle: rocksdb::DB,
}

impl BlockChainDatabase {
    /// Create a new database at the given path.
    pub fn new<P: AsRef<std::path::Path>>(path: P) -> Result<Self, rocksdb::Error> {
        let db_handle = rocksdb::DB::open_default(path)?;
        // Creating family names. TODO:: Clean this
        let opts = Options::default(); // We can tune this for performance

        match db_handle.create_cf(PROPOSER_NODE_DATA_CF, &opts) {
            Ok(_db) => {} //println!("{} created successfully", PROPOSER_NODE_DATA_CF),
            Err(e) => {
                panic!("could not create column family: {}", e);
            }
        }

        match db_handle.create_cf(VOTER_NODE_DATA_CF, &opts) {
            Ok(_db) => {} //println!("{} created successfully", VOTER_NODE_DATA_CF),
            Err(e) => {
                panic!("could not create column family: {}", e);
            }
        }

        match db_handle.create_cf(LEDGER_CF, &opts) {
            Ok(_db) => {} //println!("{} created successfully", LEDGER_CF),
            Err(e) => {
                panic!("could not create column family: {}", e);
            }
        }

        match db_handle.create_cf(PROP_TREE_LEADER_VEC_CF, &opts) {
            Ok(_db) => {} //println!("{} created successfully", PROP_TREE_LEADER_VEC_CF),
            Err(e) => {
                panic!("could not create column family: {}", e);
            }
        }
        match db_handle.create_cf(PROP_TREE_PROP_BLOCKS_CF, &opts) {
            Ok(_db) => {} //println!("{} created successfully", PROP_TREE_PROP_BLOCKS_CF),
            Err(e) => {
                panic!("could not create column family: {}", e);
            }
        }

        return Ok(BlockChainDatabase { handle: db_handle });
    }

    /// Insert into the database.
    pub fn insert<D: Serialize>(
        &self,
        cf_name: &str,
        hash: &H256,
        data: D,
    ) -> Result<(), rocksdb::Error> {
        let hash_u8: [u8; 32] = hash.into();
        let cf = self.handle.cf_handle(cf_name).unwrap();
        let serialized_data = serialize(&data).unwrap();
        return self.handle.put_cf(cf, &hash_u8, &serialized_data);
    }

    //TODO: Check the key without getting the value (Use Bloom filters maybe?)
    pub fn check(&self, cf_name: &str, hash: &H256) -> Result<bool, rocksdb::Error> {
        let hash_u8: [u8; 32] = hash.into();
        let cf = self.handle.cf_handle(cf_name).unwrap();
        let serialized = self.handle.get_cf(cf, &hash_u8)?;
        match serialized {
            None => return Ok(false),
            Some(_s) => return Ok(true),
        }
    }

    pub fn delete(&self, cf_name: &str, hash: &H256) -> Result<(), rocksdb::Error> {
        let hash_u8: [u8; 32] = hash.into();
        let cf = self.handle.cf_handle(cf_name).unwrap();
        return self.handle.delete_cf(cf, &hash_u8);
    }
}

#[cfg(test)]
mod tests {}