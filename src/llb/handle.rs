//Importing standard libraries
use std::{
    io::{prelude::*, BufReader},
    time::Instant,
    sync::{Arc},
    net::TcpStream,
};

use super::{
    datastr::DataBase
};
// , store: Arc<DataBase>
pub fn handle(mut stream: TcpStream) -> u8 {
    let timestart = Instant::now();
    let mut buffer = BufReader::new(stream.try_clone().unwrap());
    // let dt = String::from("test");

    stream.write("Successfully connected to Hermod. Opening CLI...\n\n[Hermod]> ".as_bytes()).unwrap();
    println!("Started handle in {:.3?}", timestart.elapsed());

    loop {
        let mut linebuf = String::from("");
        buffer.read_line(&mut linebuf);
        let chrono = Instant::now();
        println!("Got this: {}", linebuf);

        let response = format!("{}{:.3?}{}", "\nQuery completed in ", chrono.elapsed(), "\n\n");
        println!("{}", response);
        stream.write(response.as_bytes());
        stream.write("[Hermod]> ".as_bytes());
    }

    println!("Query chronometer: {:.2?}", timestart.elapsed());

    return 0;
}