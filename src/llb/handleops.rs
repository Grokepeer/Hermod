//Importing standard libraries
use std::{
    io::Write,
    net::TcpStream,
    time::Instant,
    sync::Arc
};

use super::{
    datastr::DataBase
};

pub fn getop(query: &str, store: &Arc<DataBase>, mut stream: &TcpStream) {
    let timestart = Instant::now();

    let l = match query.find(" ") {
        Some(n) => n + 1,
        _ => return,
    };

    println!("Key: {:?}", &query[..l - 1].as_bytes());

    if &query[l..l + 4] == "from" {
        match store.get_table(&query[l + 5..query.len() - 1]) {
            Ok(table) => {
                println!("Table here: {:.2?}", timestart.elapsed());
                match table.get_record(&query[..l - 1]) {
                    Ok(keydata) => {
                        println!("Record here: {:.2?}", timestart.elapsed());
                        match keydata.data.read() {
                            Ok(data) => {
                                println!("Data here: {:.2?}", timestart.elapsed());
                                // stream.write(data.as_bytes()).unwrap_or(0);
                                0
                            },
                            Err(_) => 0
                            // stream.write("Unable to access Data".as_bytes()).unwrap_or(0)
                        };
                    },
                    Err(_) => ()
                    // stream.write("No KeyData with the given key".as_bytes()).unwrap_or(0)
                };
            },
            Err(_) => ()
            // stream.write("No DataTable with the given name".as_bytes()).unwrap_or(0)
        }
    } else {
        stream.write("Invalid parameters".as_bytes()).unwrap_or(0);
    }
}

pub fn setop(query: &str, store: &Arc<DataBase>, mut stream: &TcpStream) {
    let l = match query.find(" ") {
        Some(n) => n + 1,
        _ => return,
    };

    if &query[l..l + 2] == "in" {
        let l2 = match &query[l + 3..].find(" ") {
            Some(n) => l + n + 4,
            _ => return,
        };

        if &query[l2..l2 + 2] == "to" {
            match store.get_table(&query[l + 3..l2 - 1]) {
                Ok(table) => match table.create_record(&query[..l - 1], &query[l2 + 3..]) {
                    0 => 0,
                    // stream.write("KeyData set successfully".as_bytes()).unwrap_or(0),
                    1 => 0,
                    // stream.write("KeyData already set".as_bytes()).unwrap_or(0),
                    _ => 0
                    // stream.write("Unable to create KeyData".as_bytes()).unwrap_or(0)
                },
                Err(_) => stream.write("No DataTable with the given name".as_bytes()).unwrap_or(0)
            };
        }
    } else {
        stream.write("Invalid parameters".as_bytes()).unwrap_or(0);
    }
}

pub fn delop(query: String, store: &Arc<DataBase>, mut stream: &TcpStream) {
    // if query.len() == 4 && query[2] == "from" {
    //     match store.get_table(query[3]) {
    //         Ok(table) => match table.delete_record(query[1]) {
    //             0 => stream.write("KeyData deleted successfully".as_bytes()).unwrap_or(0),
    //             1 => stream.write("KeyData doesn't exists".as_bytes()).unwrap_or(0),
    //             _ => stream.write("Unable to delete KeyData".as_bytes()).unwrap_or(0)
    //         },
    //         Err(_) => stream.write("No DataTable with the given name".as_bytes()).unwrap_or(0)
    //     };
    // } else {
    //     stream.write("Invalid parameters".as_bytes()).unwrap_or(0);
    // }
}

pub fn getlen(query: &str, store: &Arc<DataBase>, mut stream: &TcpStream) {
    match store.get_table(&query[7..query.len() - 1]) {
        Ok(table) => match table.table.read() {
            Ok(vec) => {
                // stream.write(&vec.len().to_be_bytes()).unwrap_or(0);
                println!("{}", vec.len());
                0
            },
            Err(_) => stream.write("Unable to access table".as_bytes()).unwrap_or(0)
        },
        Err(_) => stream.write("No DataTable with the given name".as_bytes()).unwrap_or(0)
    };
}

pub fn supercreate(query: String, store: &Arc<DataBase>, mut stream: &TcpStream) {

}

pub fn superdelete(query: String, store: &Arc<DataBase>, mut stream: &TcpStream) {

}