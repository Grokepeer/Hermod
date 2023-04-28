//Importing standard libraries
use std::{
    io::Write,
    net::TcpStream,
    sync::Arc
};

use super::{
    datastr::DataBase
};

//Get operation requires a query with "[data-key] from [datatable]"
pub fn getop(query: &str, store: &Arc<DataBase>, mut stream: &TcpStream, auth: &bool) -> u16 {
    let l = match query.find(" ") { //Finds the space between [data-key] and "from"
        Some(n) => n + 1,
        _ => return 400,
    };

    let querylen = query.len();
    if querylen > l + 6 && &query[l..l + 4] == "from" { //Checks that there's a "from" and something after it (the datatable name)
        match store.get_table(&query[l + 5..query.len() - 1]) {
            Ok(table) => match table.get_record(&query[..l - 1]) {
                Ok(data) => {
                    stream.write(data.as_bytes()).unwrap_or(0);
                    return 200;
                },
                Err(_) => return 404
            },
            Err(_) => return 404
        }
    }
    return 400
}

//Set operation requires a query with "[data-key] in [datatable] to [data]"
//Returns 409 if the record already existed and was not overidden, 200 if it was successfully created
pub fn setop(query: &str, store: &Arc<DataBase>, mut _stream: &TcpStream, auth: &bool) -> u16 {
    let l = match query.find(" ") { //Finds the space between [data-key] and "from"
        Some(n) => n + 1,
        _ => return 400,
    };

    let querylen = query.len();
    if querylen > l + 4 && &query[l..l + 2] == "in" {   //Checks that there's a "in" and the datatable name after it
        let l2 = match &query[l + 3..].find(" ") {  //Finds the space between [datatable] and "to"
            Some(n) => l + n + 4,
            _ => return 400,
        };

        if querylen > l2 + 4 && &query[l2..l2 + 2] == "to" {    //Checks that there's a "to" and some data after it
            return match store.get_table(&query[l + 3..l2 - 1]) {
                Ok(table) => match table.create_record(&query[..l - 1], &query[l2 + 3..], auth) {
                    0 => 201,
                    1 => 405,
                    2 => 200
                    _ => 500
                },
                Err(_) => 404
            };
        }
    }
    return 400
}

//Del operation requires a query with "[data-key] from [datatable]"
pub fn delop(query: &str, store: &Arc<DataBase>, mut _stream: &TcpStream, auth: &bool) -> u16 {
    if !auth {
        return 405
    }
    
    let l = match query.find(" ") { //Finds the space between [data-key] and "from"
        Some(n) => n + 1,
        _ => return 400,
    };

    let querylen = query.len();
    if querylen > l + 6 && &query[l..l + 4] == "from" { //Checks that there's a "from" and the datatable name after it
        match store.get_table(&query[l + 5..query.len() - 1]) {
            Ok(table) => match table.delete_record(&query[..l - 1]) {
                0 => return 200,
                1 => return 404,
                _ => return 500
            },
            Err(_) => return 404
        }
    }
    return 400
}

//Given a DataTable this function writes to stream the number of elements present in it's data vector
pub fn getlen(query: &str, store: &Arc<DataBase>, mut stream: &TcpStream) -> u16 {
    match store.get_table(&query[..query.len() - 1]) {
        Ok(table) => match table.table.read() {
            Ok(vec) => {
                stream.write(vec.len().to_string().as_bytes()).unwrap_or(0);
                return 200
            },
            Err(_) => return 500
        },
        Err(_) => return 404
    };
}

//Returns all the tables in the DB in a list
pub fn gettab(query: &str, store: &Arc<DataBase>, mut stream: &TcpStream) -> u16 {
    if query.len() == 0 {
        let mut tablevec = Vec::new();
        for table in store.db.read().unwrap().iter() {
            tablevec.push(table.key.to_string());
        }
        stream.write(format!("{:?}", tablevec).as_bytes()).unwrap_or(0);
    }
    return 200
}

//Creates a new table in the DB
//Returns 400 if the table already existed
pub fn supercreate(query: &str, store: &Arc<DataBase>, mut _stream: &TcpStream) -> u16 {
    if query.len() > 1 {
        match store.create_table(query.trim()) {
            0 => return 200,
            1 => return 400,
            _ => return 500
        }
    }
    return 400
}

//Deletes a table from the DB
pub fn superdelete(query: &str, store: &Arc<DataBase>, mut _stream: &TcpStream) -> u16 {
    if query.len() > 1 {
        match store.delete_table(query.trim()) {
            0 => return 200,
            1 => return 404,
            _ => return 500
        };
    }
    return 400
}