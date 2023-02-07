// YBD Front-End Node.js APP
// Copyright(c) 2022-2023 Matteo Minardi <contact@ybdrink.com>
// AGPL Licensed

//Importing standard libraries
use std::{
    thread,
    time,
    str,
    io::{prelude::*},
    net::{TcpListener, TcpStream},
    sync::{Arc, RwLock, Mutex}
};

//Importing libraries structs and functions
use Hermod::llb::{
    threads::ThreadPool,
    threads::KeyData
};

fn main() {
    let W = 4;  //HTTP server thread count
    println!("[Hermod] Up and running...");
    println!("[Hermod] Hermod settings:\n - Core Count:\t{W}\n - ANN Optimization:\t disabled");

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
                let store_clone = Arc::clone(&store);
                pool.execute(|| { handle(stream, store_clone) });    //Sends the job off to the ThreadPool
            }
            Err(_) => {
                println!("[Hermod] Stream error when accepting connection.")
            }
        }
    }

    println!("[Hermod] Shutting down.");
}

fn handle(mut stream: TcpStream, store: Arc<RwLock<Vec<KeyData>>>) {
    let mut req: Vec<u8> = Vec::new();  //Vector containing the complete HTTP request (groups together all buffers)
    let mut cl = 0; //Content-Length
    let mut clcheck: bool = false; //Content-Length has-been-acquired check

    let badreq = "HTTP/1.1 400 Bad Request\r\nBad Request";

    loop {   //The handle reads data by buffers of 500 bytes, if the first doesn't contain the required headers it drops the request
        println!("Loop");
        let mut buffer = [0; 250];
        match stream.read(&mut buffer) {    //Reads from the stream the first buffer of requests
            Ok(n) => {
                if !clcheck {    //If there's no Content-Length defined yet it will search for it in the buffer received
                    println!("clcheck false");
                    let reqlines: Vec<_> = match str::from_utf8(&buffer) {  //Casts the u8 slice received from the stream.read() to UTF8 string
                        Ok(content) => { content.lines().collect() }    //If the casting goes well it slices the headers in lines to read each
                        Err(_) => { 
                            stream.write_all(badreq.as_bytes());
                            return 
                        }
                    };
                    
                    for line in reqlines {  //Reads each line of the casted request
                        if line.starts_with("Content-Length") { //Gets the Content-Length
                            let tcl: Vec<&str> = line.split(":").collect();
                            println!("Got content-length here");
                            cl = match tcl[1].trim().parse() {  //Casts CT to u8, if it fails the content length is considered 0
                                Ok(cl) => { 
                                    clcheck = true;
                                    cl 
                                }
                                Err(_) => { 
                                    0 
                                }
                            };
                        }
                    }
                }

                req.extend_from_slice(&buffer); //Extends the vector containing the complete request
                let reqslice: Vec<&str> = str::from_utf8(&req).unwrap().split("\r\n\r\n").collect();
                let bodylen = match reqslice[1].len() {
                    Ok(len) => { len }
                    None => { 0 }
                };
                if bodylen >= cl {   //If the body (the request part after the double new line) len() is longer than the declared Content-Length it stops reading
                    println!("Break");
                    break
                }
            }
            Err(_) => {
                stream.write_all(badreq.as_bytes());
                return
            }
        }
    }

    println!("Got content length {cl}");

    println!("{:?}", str::from_utf8(&req).unwrap());
    // println!("{:?}", req);

    // thread::sleep(time::Duration::from_millis(4000));

    println!("Store first value: {}", store.read().unwrap()[0].pair.lock().unwrap());
    store.write().unwrap().push({ KeyData {
        key: String::from("Bitch"),
        pair: Mutex::new(String::from("Fuck all this informations... you get nothing"))
    }});
    
    let (status, res_content) = ("HTTP/1.1 200 OK", "Got it");
    let length = res_content.len();
    let response = format!("{status}\r\nContent-Length: {length}\r\n\r\n{res_content}");
    stream.write_all(response.as_bytes()).unwrap();
}