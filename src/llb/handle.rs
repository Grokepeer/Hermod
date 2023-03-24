//Importing standard libraries
use std::{
    io::{prelude::*, BufReader},
    time::Instant,
    sync::Arc,
    net::{TcpStream, Shutdown},
};

use super::{
    datastr::{DataBase, PkgData},
    handleops::{getop, setop, delop, getlen, supercreate, superdelete}
};

pub fn handle(mut stream: TcpStream, id: u8, store: Arc<DataBase>, pkg: Arc<PkgData>) {
    let timestart = Instant::now();
    let mut buffer = BufReader::new(stream.try_clone().unwrap());

    stream.write("Welcome to Hermod v0.2.0\n> ".as_bytes()).unwrap();
    println!("Started handle ID.{id} in {:.3?}", timestart.elapsed());

    loop {
        let mut query = String::from("");
        match buffer.read_line(&mut query) {
            Err(_) => break,
            _ => {}
        };

        let chrono = Instant::now();    //Starts the query

        let query = query.replace("\n", "");
        let query: Vec<_> = query.split(" ").collect();

        // println!("{:?}", query);

        match query[0] {
            "get" => getop(query, &store, &stream),
            "set" => setop(query, &store, &stream),
            "del" => delop(query, &store, &stream),
            "super" => superhandle(query, &store, &stream),
            "exit" => break,
            _ => {}
        };

        if true {
            let querytime = format!("{}{:.3?}{}", "\nQuery completed in ", chrono.elapsed(), "\n> ");   //End the query
            stream.write(querytime.as_bytes()).unwrap_or(0);
        }
    }

    stream.write("\nSuccessfully dropping the connection to Hermod...".as_bytes()).unwrap_or(0);
    println!("Closed handle ID.{id} after {:.3?}", timestart.elapsed());
    stream.shutdown(Shutdown::Read).unwrap_or(());
}

fn superhandle (query: Vec<&str>, store: &Arc<DataBase>, mut stream: &TcpStream) {
    if query.len() > 1 {
        match query[1] {
            "getlen" => getlen(query, &store, &stream),
            _ => {}
        };
    } else {
        stream.write("No operation specified".as_bytes()).unwrap_or(0);
    }
}