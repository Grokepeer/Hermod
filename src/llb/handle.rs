//Importing standard libraries
use std::{
    io::{prelude::*, BufReader},
    time::Instant,
    sync::Arc,
    net::{TcpStream, Shutdown},
};

use super::{
    datastr::{DataBase, PkgData},
    handleops::{getop, setop, delop, getlen, gettab, supercreate, superdelete}
};

pub fn handle(mut stream: TcpStream, id: u8, store: Arc<DataBase>, pkg: Arc<PkgData>) {
    let timestart = Instant::now();
    let mut buffer = BufReader::new(stream.try_clone().unwrap());

    //Connection established, send confirmation to the Client
    stream.write(format!("Hermod - Connection established (v{}, v{})", pkg.pkgv, pkg.apiv).as_bytes()).unwrap();
    println!("Started handle ID.{id} in {:.3?}", timestart.elapsed());

    //Starts CLI loop
    loop {
        let mut query = String::from("");
        match buffer.read_line(&mut query) {    //Read line from the TCP buffer
            Err(_) => break,
            _ => {}
        };

        //Starts the query
        let chrono = Instant::now();

        let mut code = 400u16;
        let querylen = query.len();
        
        //Checks that the query at leasts has the first 3 character (1st parameter)
        if querylen > 2 {
            
            let mut nxt = "";
            if querylen > 4 {
                nxt = &query[4..];
            }

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

    //Closing TCP connection
    stream.write("\nDropping the connection to Hermod...".as_bytes()).unwrap_or(0);
    println!("Closed handle ID.{id} after {:.3?}", timestart.elapsed());
    stream.shutdown(Shutdown::Read).unwrap_or(());
}

fn superhandle (query: &str, store: &Arc<DataBase>, stream: &TcpStream) -> u16 {
    let querylen = query.len();
    
    if querylen > 5 {
            
        let mut nxt = "";
        if querylen > 7 {
            nxt = &query[7..];
        }

        return match &query[..6] {
            "create" => supercreate(nxt, &store, &stream),
            "delete" => superdelete(nxt, &store, &stream),
            "gettab" => gettab(nxt, &store, &stream),
            "getlen" => getlen(nxt, &store, &stream),
            _ => 400
        };
    }
    return 400
}