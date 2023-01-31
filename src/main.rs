use std::{
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};

use Hermod::llb::{
    tmodule::ThreadPool,
    jmodule::Json,
};

fn main() {
    println!("[Hermod] Up and running...");

    let listener = TcpListener::bind("0.0.0.0:2088").expect("[Hermod] Unable to bind to port 2088 on host");
    let pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        });
    }

    println!("[Hermod] Shutting down.");
}

fn handle_connection(mut stream: TcpStream) {
    //Saves in a buffer the request from the TCPStream buffer and then saves its line in a vector names req.
    let mut buffer = [0; 512];
    let rawreq = stream.read(&mut buffer).unwrap();

    // let buffer = BufReader::new(&mut stream);

    // let rawreq = String::from(buffer);
    let req = rawreq;

    // Collects all lines from the http request
    // let reqlines: Vec<_> = req
    // .lines()
    // .collect();

    let test = Json::from("Gotcha");
    println!("Result of JSON.read(): {}", String::from(buffer));

    println!("{:?}", req);
    
    // Finds the request line with the Content-Length and saves it in the var size..
    // let split: Vec<_> = req.split("\r\n\r\n").collect();

    // println!("{:?}", split[1].replace("\0", ""));

    // Matches the header of the http request
    // let (status, res_content) = match reqlines[0] {
    //     "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "Got it"),
    //     "GET /sleep HTTP/1.1" => ("HTTP/1.1 200 OK", "Sleep"),
    //     _ => ("HTTP/1.1 404 NOT FOUND", "404")
    // };
    
    let (status, res_content) = ("HTTP/1.1 200 OK", "Got it");
    let length = res_content.len();
    let response = format!("{status}\r\nContent-Length: {length}\r\n\r\n{res_content}");
    stream.write_all(response.as_bytes()).unwrap();
}