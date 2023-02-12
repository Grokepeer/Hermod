//Importing standard libraries
use std::{
    str,
    io::{Write, Read},
    sync::{Arc, RwLock, Mutex}
};

//Data structure used in the KeysVector, points to another space in the heap that contains all data paired with the key in a String
pub struct KeyData {
    key: String,
    data: Mutex<String>
}

pub struct DataTable {
    key: String,
    table: RwLock<Vec<Arc<KeyData>>>
}

pub struct DataBase(RwLock<Vec<Arc<DataTable>>>);

impl DataBase {
    pub fn new() -> DataBase {
        let mut db = DataBase(RwLock::new(Vec::new()));
        db.createTable("_basedb");

        db
    }

    pub fn getOpTable(self, tablename: &str) -> Result<&Arc<DataTable>, &'static str> {
        for table in self.0.write().unwrap().iter() {
            if table.key.as_str() == tablename {
                return Ok(table)
            }
        }

        Err("no result")
    }

    pub fn getTable(self, tablename: &str) -> Result<&Arc<DataTable>, &'static str> {
        for table in self.0.read().unwrap().iter() {
            if table.key.as_str() == tablename {
                return Ok(table)
            }
        }

        Err("no result")
    }

    pub fn createTable(self, tablename: &str) {
        self.0.write().unwrap().push(Arc::new(DataTable::new(tablename)))
    }
}

impl DataTable {
    pub fn new(tablename: &str) -> DataTable {
        let mut table = RwLock::new(Vec::new());
        table.createRecord("_base", "_data");

        DataTable {
            key: String::from(tablename),
            table
        }
    }

    pub fn getOpRecord(self, recordkey: &str) -> Result<&Arc<KeyData>, &'static str> {
        for record in self.table.write().unwrap().iter() {
            if record.key.as_str() == recordkey {
                return Ok(record)
            }
        }

        Err("no result")
    }

    pub fn getRecord(self, recordkey: &str) -> Result<&Arc<KeyData>, &'static str> {
        for record in self.table.read().unwrap().iter() {
            if record.key.as_str() == recordkey {
                return Ok(record)
            }
        }

        Err("no result")
    }

    pub fn createRecord(self, recordkey: &str, recordata: &str) {
        self.table.write().unwrap().push(Arc::new({ KeyData {
            key: String::from(recordkey),
            data: Mutex::new(String::from(recordata))
        }}))
    }
}