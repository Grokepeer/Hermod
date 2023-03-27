//Importing standard libraries
use std::{
    str,
    sync::{Arc, RwLock},
    mem::size_of_val,
};

pub struct PkgData {
    pub pkgv: String,
    pub apiv: String,
    pub deltoken: String
}

//Data structure used in the DB, points to another space in the heap that contains all data paired with the key in a String
pub struct KeyData {
    pub key: [u8; 20],
    pub data: RwLock<String>
}

pub struct DataTable {
    pub key: String,
    pub table: RwLock<Vec<KeyData>>
}

pub struct DataBase {
    pub db: RwLock<Vec<Arc<DataTable>>>
}

impl DataBase {
    pub fn new() -> DataBase {
        let database = { DataBase {
            db: RwLock::new(Vec::new())
        }};
        database.create_table("_basedb");

        return database
    }

    //This function looks for a table in the db given a table key and returns a pointer to the table
    //Returns Err("no result") if the table couldn't be found
    //Returns Err("server error") if the function couldn't complete the operation
    pub fn get_table(&self, tablename: &str, recordname: &str) -> Result<Arc<DataTable>, &'static str> {
        match self.db.read() {
            Ok(db) => {
                for table in db.iter() {
                    if table.key.as_str() == tablename {
                        return table.get_record(recordname);
                    }
                }
            }
            Err(_) => {
                println!("[Hermod] Unable to get db.read() access");
                return Err("server error")
            }
        }

        Err("no result")
    }

    //This function looks for a table in the db
    //Returns 1 if the table exists, 0 if it doesn't and -1 if the function couldn't complete the operation correctly
    fn is_table(&self, tablename: &str) -> i8 {
        match self.db.read() {
            Ok(db) => {
                for table in db.iter() {
                    if table.key.as_str() == tablename {
                        return 1
                    }
                }
            }
            Err(_) => {
                println!("[Hermod] Unable to get db.read() access");
                return -1
            }
        }
        
        return 0
    }

    //This function creates a table in the db that it was called upon
    //Returns 0 if the operation was successful, 1 if the table already existed, -1 if the function couldn't complete properly
    pub fn create_table(&self, tablename: &str) -> i8 {
        match self.is_table(tablename) {
            0 => {
                match self.db.write() {
                    Ok(mut db) => {
                        db.push(Arc::new(DataTable::new(tablename)));
                        return 0
                    }
                    Err(_) => {
                        println!("[Hermod] Unable to get db.write() access");
                        return -1
                    }
                }
            }
            r => return r
        }
    }
}

impl DataTable {
    pub fn new(tablename: &str) -> DataTable {
        let datatable = { DataTable {
            key: String::from(tablename),
            table: RwLock::new(Vec::new())
        }};
        datatable.create_record("_base", "_data");

        return datatable
    }

    //This function looks for a record (given a record key) in the table that the function was called upon and returns a pointer to the record
    //Returns Err("no result") if the record couldn't be found
    //Returns Err("server error") if the function couldn't complete the operation
    pub fn get_record(&self, recordkey: &str) -> Result<&'a str, &'static str> {
        match self.table.read() {
            Ok(table) => {
                for record in table.iter() {
                    if compare(recordkey.as_bytes(), &record.key) {
                        println!("Address: {:p}", record);
                        println!("Address key: {:?}", record.key);
                        println!("Address data: {}", size_of_val(&*record.data.read().unwrap()));
                        return Ok(record.data.read())
                    }
                }
            }
            Err(_) => {
                println!("[Hermod] Unable to get table.read() access");
                return Err("server error")
            }
        }

        Err("no result")
    }

    //This function looks for a record in the table it was called upon
    //Returns 1 if the record exists, 0 if it doesn't and -1 if the function couldn't complete the operation correctly
    fn is_record(&self, recordkey: &str) -> (i8, u32) {
        match self.table.read() {
            Ok(table) => {
                let mut x = 0;
                for record in table.iter() {
                    if compare(recordkey.as_bytes(), &record.key) {
                        return (1, x)
                    }
                    x += 1;
                }
            }
            Err(_) => {
                println!("[Hermod] Unable to get table.read() access");
                return (-1, 0)
            }
        }
        
        return (0, 0)
    }

    //This function creates a record in the table that it was called upon
    //Returns 0 if the operation was successful, 1 if the record already existed, -1 if the function couldn't complete properly
    pub fn create_record(&self, recordkey: &str, recordata: &str) -> i8 {
        let mut byteskey = [0u8; 20];
        for i in 0..recordkey.len() {
            byteskey[i as usize] = recordkey.as_bytes()[i as usize];
        }

        println!("Set key: {:?}", byteskey);
        match self.is_record(recordkey) {
            (0, _x) => {
                match self.table.write() {
                    Ok(mut table) => {
                        table.push(Arc::new({ KeyData {
                            key: byteskey,
                            data: RwLock::new(String::from(recordata))
                        }}));
                        return 0
                    }
                    Err(_) => {
                        println!("[Hermod] Unable to get table.write() access");
                        return -1
                    }
                }
            }
            (r, _x) => return r
        }
    }

    //This function deletes a record from the table that it was called upon
    //Returns 0 if the operation was successfull, 1 if the record didn't exist, -1 if the function couldn't complete properly
    pub fn delete_record(&self, recordkey: &str) -> i8 {
        match self.is_record(recordkey) {
            (1, x) => {
                match self.table.write() {
                    Ok(mut table) => {
                        table.remove(x as usize);
                        return 0
                    }
                    Err(_) => {
                        println!("[Hermod] Unable to get table.write() access");
                        return -1
                    }
                }
            }
            (_r, _x) => return 1
        }
    }
}

fn compare(slice: &[u8], array: &[u8; 20]) -> bool {
    for i in 0..slice.len() {
        if slice[i] != array[i] {
            return false
        }
    }

    return true
}