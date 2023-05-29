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

    let mut query = String::from("");   //Get DEL_TOKEN
    let auth = match buffer.read_line(&mut query) {
        Err(_) => false,
        _ => pkg.deltoken == query[6..query.len() - 1]  //Checks if client DEL_TOKEN matches the Pkg DEL_TOKEN
    };

    //Connection established, send confirmation to the Client
    stream.write(format!("Hermod - Connection established (v{}, v{}, {})", pkg.pkgv, pkg.apiv, if auth { "Auth" } else { "noAuth" }).as_bytes()).unwrap();
    println!("Started handle ID.{id} in {:.3?}", timestart.elapsed());

    //Starts CLI loop
    loop {
        let mut bytes: Vec<u8> = Vec::new();
        match buffer.read_until(0x4, &mut bytes) {    //Read line from the TCP buffer
            Err(_) => break,
            _ => {}
        };
        let query = String::from_utf8(bytes).unwrap();

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

            println!("Query: {:?}", query);

            code = match &query[..3] {
                "get" => getop(nxt, &store, &stream, &auth),
                "set" => setop(nxt, &store, &stream, &auth),
                "del" => delop(nxt, &store, &stream, &auth),
                "sup" => superhandle(nxt, &store, &stream, &auth),
                "ext" => break,
                _ => 400
            };
        }

        stream.write(format!("{}{:12?}{}{:3?}{}", "{", chrono.elapsed().as_nanos(), " ", code, "}\u{4}").as_bytes()).unwrap_or(0);
    }

    //Closing TCP connection
    stream.write("\nDropping the connection to Hermod...".as_bytes()).unwrap_or(0);
    println!("Closed handle ID.{id} after {:.3?}", timestart.elapsed());
    stream.shutdown(Shutdown::Read).unwrap_or(());
}

fn superhandle (query: &str, store: &Arc<DataBase>, stream: &TcpStream, auth: &bool) -> u16 {
    if !auth {  //Checks authentication to super user
        return 403
    }

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