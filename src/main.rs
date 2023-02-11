// YBD Front-End Node.js APP
// Copyright(c) 2022-2023 Matteo Minardi <contact@ybdrink.com>
// AGPL Licensed

//Importing standard libraries
use std::{
    env,
    str,
    net::{TcpListener, TcpStream},
    sync::{Arc, RwLock, Mutex}
};

//Importing libraries structs and functions
use hermod::llb::{
    threads::ThreadPool,
    threads::KeyData,
    handle::handle,
};

fn main() {
    let mut w = 1;  //HTTP server thread count
    let mut deltoken = Arc::new(String::from("token"));

    let variables = env::vars();    //Gets all environment variables
    for (key, value) in variables.into_iter() {
        if key == "HTTP_Threads" {
            w = value.parse().unwrap()
        }
        if key == "Del_Token" {
            deltoken = Arc::new(value)
        }
    }
    
    let listener = TcpListener::bind("0.0.0.0:2088").expect("[Hermod] Unable to bind to port 2088 on host");
    let pool = ThreadPool::new(w);  //New ThreadPool requested with worker count N
    
    //Declaration of the KeysVector, it holds all keys to all content of DB, it's set in Arc and RwLock so it can be read by many, modified by one
    let store: Arc<RwLock<Vec<Arc<KeyData>>>> = Arc::new(RwLock::new(Vec::new()));
    
    //Initializing the store vector, if the vector is not initialized mpsc channels locks will panick at empty content
    store.write().expect("[Hermod] An error occured when allocating memory to the main KeysVector").push(Arc::new({ KeyData {
        key: String::from("_base"),
        pair: Mutex::new(String::from("_base")),
    }}));
    
    println!("[Hermod] Up and running...");
    println!("[Hermod] HTTP Server threads: {w}");
    println!("[Hermod] Del_Token: {deltoken}");
    
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let store_clone = Arc::clone(&store);
                let deltoken_clone = Arc::clone(&deltoken);
                pool.execute(|| { handle(stream, store_clone, deltoken_clone) });    //Sends the job off to the ThreadPool
            }
            Err(_) => {
                println!("[Hermod] Stream error when accepting connection.")
            }
        }
    }

    println!("[Hermod] Shutting down.");
}