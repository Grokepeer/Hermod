// YBD Front-End Node.js APP
// Copyright(c) 2022-2023 Matteo Minardi <contact@ybdrink.com>
// AGPL Licensed

//Importing standard libraries
use std::{
    io,
    io::Write,
    env,
    str,
    net::{TcpListener, TcpStream},
    sync::{Arc, RwLock}
};

//Importing libraries structs and functions
use hermod::llb::{
    datastr::DataBase,
    threads::ThreadPool,
    handle::handle
};

fn main() {
    let w = option_env!("HTTP_Threads").unwrap_or("1").parse().unwrap();    //HTTP server thread count
    let deltoken = Arc::new(String::from(option_env!("Del_Token").unwrap_or("token")));
    let apiversion = option_env!("CARGO__VERSION").unwrap_or("0.0.1");
    let version = option_env!("CARGO_PKG_VERSION").unwrap_or("0.0.1");
    let stdin = io::stdin();
    
    let pool = ThreadPool::new(w);  //New ThreadPool requested with worker count N
    
    //Declaration of the KeysVector, it holds all keys to all content of DB, it's set in Arc and RwLock so it can be read by many, modified by one
    let store = Arc::new(DataBase::new());
    // println!("Get this: {}", store.get_table("_basedb").unwrap().get_record("_base").unwrap().data.read().unwrap());
    store.get_table("_basedb").unwrap().create_record("testkey", "datainside");
    // println!("This is: {}", store.get_table("_basedb").unwrap().get_record("testkey").unwrap().data.read().unwrap());
    
    println!("[Hermod] Hermod v{version}, API v{apiversion}");
    println!("[Hermod] HTTP Server threads: {w}");
    println!("[Hermod] Del_Token: {deltoken}");
    println!("[Hermod] Hermod is starting up... Wait for the CLI to start");
    
    loop {
        print!("> ");
        io::stdout().flush().unwrap();
        let mut userinput = String::from("");
        stdin.read_line(&mut userinput);
        println!("Input: {userinput}");

        let store_clone = Arc::clone(&store);
        handle(userinput, store_clone);
        // pool.execute(|| { handle(userinput, store_clone) });    //Sends the job off to the ThreadPool
    }

    println!("[Hermod] Shutting down.");
}