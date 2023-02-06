// YBD Front-End Node.js APP
// Copyright(c) 2022-2023 Matteo Minardi <contact@ybdrink.com>
// AGPL Licensed

//Importing standard libraries
use std::{
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    sync::{Arc, RwLock, Mutex}
};

//Importing libraries structs and functions
use Hermod::llb::{
    threads::ThreadPool,
    threads::KeyData
};

use http::Request;
use serde::de::Deserialize;

fn main() {
    let W = 4;  //HTTP server thread count
    println!("[Hermod] Up and running...");
    println!("[Hermod} Hermod settings:\n - Core Count:\t{W}\n - ANN Optimization:\t disabled");

    let listener = TcpListener::bind("0.0.0.0:2088").expect("[Hermod] Unable to bind to port 2088 on host");
    
    let pool = ThreadPool::new(W);  //New ThreadPool requested with worker count N

    //Declaration of the KeysVector, it holds all keys to all content of DB, it's set in Arc and RwLock so it can be read by many, modified by one
    let store = Arc::new(RwLock::new(Vec::new()));

    //Initializing the store vector, if the vector is not initialized mpsc channels locks will panick at empty content
    store.write().expect("[Hermod] An error occured when allocating memory to the main KeysVector").push({ KeyData {
        key: String::from("_base"),
        pair: Mutex::new(String::from("_base")),
    }});

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                pool.execute(|| { handle(stream); }, Arc::clone(&store));    //Sends the job off to the ThreadPool
            }
            Err(_) => {
                println!("[Hermod] Stream error when accepting connection.")
            }
        }
    }

    println!("[Hermod] Shutting down.");
}

fn handle(mut stream: TcpStream) {
    //Saves in a buffer the request from the TCPStream buffer and then saves its line in a vector names req.
    let mut buffer = [0; 500];
    let rawreq = stream.read(&mut buffer).unwrap();

    // let buffer = BufReader::new(&mut stream);

    // let rawreq = String::from(buffer);
    let req = rawreq;

    // Collects all lines from the http request
    // let reqlines: Vec<_> = req
    // .lines()
    // .collect();

    println!("Result of JSON.read(): {}", req);

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

    let request = Request::builder()
    .method("GET")
    .uri("https://www.rust-lang.org/")
    .header("X-Custom-Foo", "Bar")
    .body(())
    .unwrap();
    
    let (status, res_content) = ("HTTP/1.1 200 OK", "Got it");
    let length = res_content.len();
    let response = format!("{status}\r\nContent-Length: {length}\r\n\r\n{res_content}");
    stream.write_all(response.as_bytes()).unwrap();
}