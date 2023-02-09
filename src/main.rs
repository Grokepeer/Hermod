// YBD Front-End Node.js APP
// Copyright(c) 2022-2023 Matteo Minardi <contact@ybdrink.com>
// AGPL Licensed

//Importing standard libraries
use std::{
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
    let w = 4;  //HTTP server thread count
    println!("[Hermod] Up and running...");
    println!("[Hermod] Hermod settings:\n - Core Count:\t{w}\n - ANN Optimization:\t disabled");

    let listener = TcpListener::bind("0.0.0.0:2088").expect("[Hermod] Unable to bind to port 2088 on host");
    
    let pool = ThreadPool::new(w);  //New ThreadPool requested with worker count N

    //Declaration of the KeysVector, it holds all keys to all content of DB, it's set in Arc and RwLock so it can be read by many, modified by one
    let store: Arc<RwLock<Vec<Arc<KeyData>>>> = Arc::new(RwLock::new(Vec::new()));

    //Initializing the store vector, if the vector is not initialized mpsc channels locks will panick at empty content
    store.write().expect("[Hermod] An error occured when allocating memory to the main KeysVector").push(Arc::new({ KeyData {
        key: String::from("_base"),
        pair: Mutex::new(String::from("_base")),
    }}));

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

fn handle(mut stream: TcpStream, store: Arc<RwLock<Vec<Arc<KeyData>>>>) {
    let mut req = String::from("");  //Vector containing the complete HTTP request (groups together all buffers)
    let mut cl = 0; //Content-Length
    let mut key = String::from("");    //Data Key
    let mut deltoken = String::from("");  //Destruction token
    let mut httpheader = String::from("");
    let mut reqbody = String::from("");
    let mut clcheck: bool = false; //Content-Length has-been-acquired check

    let dt = "deltoken";

    let badreq = "HTTP/1.1 400 Bad Request\r\nBad Request";

    loop {   //The handle reads data by buffers of 500 bytes, if the first doesn't contain the required headers it drops the request
        // println!("Loop");
        let mut buffer = [0; 500];
        match stream.read(&mut buffer) {    //Reads from the stream the first buffer of requests
            Ok(_) => {
                let reqstring = str::from_utf8(&buffer).unwrap();

                if !clcheck {    //If there's no Content-Length defined yet it will search for it in the buffer received
                    // println!("clcheck false");
                    let reqlines: Vec<_> = reqstring.lines().collect(); //It slices the headers in lines to read each
                    httpheader = String::from(reqlines[0]);
                    
                    for line in reqlines {  //Reads each line of the casted request
                        if line.starts_with("Content-Length") { //Gets the Content-Length
                            let tcl: Vec<&str> = line.split(":").collect();
                            // println!("Got content-length here: {}", line);
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
                        if line.starts_with("Data-Key") {
                            let tcl: Vec<&str> = line.split(":").collect();
                            key = String::from(tcl[1].trim());
                            key = key.replace("\"", "");
                        }
                        if line.starts_with("Del-Token") {
                            let tcl: Vec<&str> = line.split(":").collect();
                            deltoken = String::from(tcl[1].trim());
                            deltoken = deltoken.replace("\"", "");
                        }
                    }
                }

                req.push_str(reqstring); //Extends the vector containing the complete request

                let mut bodylen = 0;
                if req.contains("\r\n\r\n") {   //If the request has a body it starts to count the length
                    let body: Vec<&str> = req.split("\r\n\r\n").collect();
                    bodylen = body[1].len();
                    reqbody = String::from(body[1]);
                }
                
                if bodylen >= cl {   //If the body (the request part after the double new line) len() is longer than the declared Content-Length it stops reading
                    // println!("Break");
                    break
                }
            }
            Err(_) => {
                stream.write_all(badreq.as_bytes()).unwrap();
                return
            }
        }
    }

    println!("[Hermod] Received request: {:?}", req.trim_matches(char::from(0)));

    // thread::sleep(time::Duration::from_millis(4000));

    let bodystring: String;
    let reshead;
    let resbody;
    (reshead, resbody) = match httpheader.as_str() {
        "GET /get HTTP/1.1" => {
            match find_key(&key, Arc::clone(&store)) {
                Ok(res) => {
                    bodystring = res.0.pair.lock().unwrap().to_string();
                    ("200 OK", bodystring.as_str())
                }
                Err(e) => {
                    if e == "No key found" {
                        ("404 ERROR", e)
                    } else {
                        ("500 ERROR", e)
                    }
                }
            }
        }

        "GET /set HTTP/1.1" => {
            match find_key(&key, Arc::clone(&store)) {
                Ok(res) => {
                    if deltoken.as_str() == dt {
                        *(res.0).pair.lock().unwrap() = String::from(reqbody.trim_matches(char::from(0)));
                        ("200 OK", "Record updated successfully")
                    } else {
                        ("403 Forbidden", "Unauthorized request")
                    }
                }
                Err(e) => {
                    if e == "No key found" {
                        store.write().unwrap().push(Arc::new({ KeyData {
                            key: key,
                            pair: Mutex::new(String::from(reqbody.trim_matches(char::from(0))))
                        }}));
                        ("200 OK", "Record created successfully")
                    } else {
                        ("500 ERROR", e)
                    }
                }
            }
        }

        "GET /del HTTP/1.1" => {
            match find_key(&key, Arc::clone(&store)) {
                Ok(res) => {
                    if deltoken.as_str() == dt {
                        store.write().unwrap().swap_remove(res.1);
                        ("200 OK", "Record deleted")
                    } else {
                        ("403 Forbidden", "Unauthorized request")
                    }
                }
                Err(e) => {
                    if e == "No key found" {
                        ("404 ERROR", "No record to be deleted")
                    } else {
                        ("500 ERROR", e)
                    }
                }
            }
        }
        _ => {
            println!("Unrecognized path");
            ("400 Bad Request", "Unrecognized path")
        }
    };
    
    let (status, res_content) = ("HTTP/1.1 ".to_owned() + reshead, resbody.to_string());
    let length = res_content.len();
    let response = format!("{status}\r\nContent-Length: {length}\r\n\r\n{res_content}");
    stream.write_all(response.as_bytes()).unwrap();
}

fn find_key(key: &str, store: Arc<RwLock<Vec<Arc<KeyData>>>>) -> Result<(Arc<KeyData>, usize), &'static str> {
    let storerlock = match store.read() {
        Ok(lock) => lock,
        Err(_) => {
            return Err("Couldn't get a store lock on");
        }
    };

    let mut i: usize = 0;
    for keydata in storerlock.iter() {
        if keydata.key.as_str() == key {
            return Ok((Arc::clone(&keydata), i));
        }
        i += 1;
    }

    Err("No key found")
}