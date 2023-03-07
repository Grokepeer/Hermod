//Importing standard libraries
use std::{
    thread,
    io::{prelude::*, BufReader},
    time,
    time::Instant,
    sync::Arc,
    net::{TcpStream, Shutdown},
};

use super::{
    datastr::{DataBase, PkgData}
};

pub fn handle(mut stream: TcpStream, id: &u8, store: Arc<DataBase>, pkg: Arc<PkgData>) {
    let timestart = Instant::now();
    let mut buffer = BufReader::new(stream.try_clone().unwrap());

    stream.write("Welcome to Hermod v0.2.0\n\n[Hermod]> ".as_bytes()).unwrap();
    println!("Started {id} handle in {:.3?}", timestart.elapsed());

    loop {
        let mut query = String::from("");
        match buffer.read_line(&mut query) {
            Err(_) => break,
            _ => {}
        };

        let chrono = Instant::now();

        if query.starts_with("exit") {
            break;
        } else if query.starts_with("wait") {
            thread::sleep(time::Duration::from_millis(8000));
        }

        let response = format!("{}{:.3?}{}", "\nQuery completed in ", chrono.elapsed(), "\n\n");
        stream.write(response.as_bytes()).unwrap();
        stream.write("[Hermod]> ".as_bytes()).unwrap();
    }

    stream.write("\nSuccessfully dropping connection to Hermod...".as_bytes()).unwrap();
    println!("Closed {id} handle in {:.3?}", timestart.elapsed());
    stream.shutdown(Shutdown::Read);
}