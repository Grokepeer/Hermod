//Importing standard libraries
use std::{
    io::{prelude::*, BufReader},
    time::Instant,
    sync::Arc,
    net::{TcpStream, Shutdown},
};

use super::{
    datastr::{DataBase, PkgData},
    handleops::{getop, setop, delop, getlen}
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

        let mut code: u16 = 500;
        if query.len() > 5 {
            let nxt = &query[4..];

            code = match &query[..3] {
                "get" => getop(nxt, &store, &stream),
                "set" => setop(nxt, &store, &stream),
                "del" => delop(nxt, &store, &stream),
                "sup" => superhandle(nxt, &store, &stream),
                "ext" => break,
                _ => 400
            };
        }

        stream.write(format!("{}{:12?}{}{:3?}{}", "{", chrono.elapsed().as_nanos(), " ", code, "}\n").as_bytes()).unwrap_or(0);
    }

    stream.write("\nDropping the connection to Hermod...".as_bytes()).unwrap_or(0);
    println!("Closed handle ID.{id} after {:.3?}", timestart.elapsed());
    stream.shutdown(Shutdown::Read).unwrap_or(());
}

fn superhandle (query: &str, store: &Arc<DataBase>, stream: &TcpStream) -> u16 {
    return getlen(query, &store, &stream);
}