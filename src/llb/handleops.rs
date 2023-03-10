//Importing standard libraries
use std::{
    io::{Write},
    net::{TcpStream},
    sync::{Arc}
};

use super::{
    datastr::{DataBase}
};

pub fn getop(query: Vec<&str>, store: &Arc<DataBase>, mut stream: &TcpStream) {
    if query.len() == 4 && query[2] == "from" {
        match store.get_table(query[3]) {
            Ok(table) => match table.get_record(query[1]) {
                Ok(keydata) => match keydata.data.read() {
                    Ok(data) => stream.write(data.as_bytes()),
                    Err(_) => stream.write("Unable to access Data".as_bytes())
                },
                Err(_) => stream.write("No KeyData with the given key".as_bytes())
            },
            Err(_) => stream.write("No DataTable with the given name".as_bytes())
        };
    } else {
        stream.write("Invalid parameters".as_bytes());
    }
}

pub fn setop(query: Vec<&str>, store: &Arc<DataBase>, mut stream: &TcpStream) {
    if query.len() == 6 && query[2] == "in" && query[4] == "to" {
        match store.get_table(query[3]) {
            Ok(table) => match table.create_record(query[1], query[5]) {
                0 => stream.write("KeyData set successfully".as_bytes()),
                1 => stream.write("KeyData already set".as_bytes()),
                _ => stream.write("Unable to create KeyData".as_bytes())
            },
            Err(_) => stream.write("No DataTable with the given name".as_bytes())
        };
    } else {
        stream.write("Invalid parameters".as_bytes());
    }
}

pub fn delop(query: Vec<&str>, store: &Arc<DataBase>, mut stream: &TcpStream) {

}

pub fn supercreate(query: Vec<&str>, store: &Arc<DataBase>, mut stream: &TcpStream) {

}

pub fn superdelete(query: Vec<&str>, store: &Arc<DataBase>, mut stream: &TcpStream) {

}