// YBD Front-End Node.js APP
// Copyright(c) 2022-2023 Matteo Minardi <contact@ybdrink.com>
// AGPL Licensed

//Importing standard libraries
use std::{
    env,
    str,
    sync::{Arc, RwLock}
};

//Importing libraries structs and functions
use hermod::llb::{
    datastr::DataBase,
    threads::ThreadPool,
    handle::handle
};

fn main() {
    let mut w = 1;  //HTTP server thread count
    let mut deltoken = Arc::new(String::from("token"));

    let variables = env::vars();    //Gets all environment variables
    for (key, value) in variables.into_iter() { //Cycle through env variables
        if key == "HTTP_Threads" {
            w = value.parse().unwrap()
        }
        if key == "Del_Token" {
            deltoken = Arc::new(value)
        }
    }
    
    let pool = ThreadPool::new(w);  //New ThreadPool requested with worker count N
    
    //Declaration of the KeysVector, it holds all keys to all content of DB, it's set in Arc and RwLock so it can be read by many, modified by one
    let store = Arc::new(DataBase::new());
    println!("Get this: {}", store.get_table("_basedb").unwrap().get_record("_base").unwrap().data.read().unwrap());
    store.get_table("_basedb").unwrap().create_record("testkey", "datainside");
    println!("This is: {}", store.get_table("_basedb").unwrap().get_record("testkey").unwrap().data.read().unwrap());
    
    println!("[Hermod] Up and running...");
    println!("[Hermod] HTTP Server threads: {w}");
    println!("[Hermod] Del_Token: {deltoken}");
    
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