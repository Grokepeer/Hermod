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

    stream.write(format!("Hermod - Connection established (v{}, v{})", pkg.pkgv, pkg.apiv).as_bytes()).unwrap();
    println!("Started handle ID.{id} in {:.3?}", timestart.elapsed());

    loop {
        let mut query = String::from("");
        match buffer.read_line(&mut query) {
            Err(_) => break,
            _ => {}
        };

        let chrono = Instant::now();    //Starts the query

        if query.len() > 5 {
            let op = &query[..3];  //Gets operation name (set, del, sup...)
            let nxt = &query[4..];

            // println!("{:?}", query);

            match op {
                "get" => getop(nxt, &store, &stream),
                "set" => setop(nxt, &store, &stream),
                "del" => delop(query, &store, &stream),
                "sup" => superhandle(nxt, &store, &stream),
                "ext" => break,
                _ => {}
            };
        }

        stream.write(format!("{}{:12?}{}", "{", chrono.elapsed().as_nanos(), "}").as_bytes()).unwrap_or(0);
    }

    stream.write("\nDropping the connection to Hermod...".as_bytes()).unwrap_or(0);
    println!("Closed handle ID.{id} after {:.3?}", timestart.elapsed());
    stream.shutdown(Shutdown::Read).unwrap_or(());
}

fn superhandle (query: &str, store: &Arc<DataBase>, mut stream: &TcpStream) {
    getlen(query, &store, &stream);
}