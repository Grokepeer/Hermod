// HermodDB Host
// Copyright(c) 2022-2023 Matteo Minardi <contact@ybdrink.com>
// AGPL Licensed

//Importing standard libraries
use std::{
    thread,
    net::TcpListener,
    sync::Arc
};

//Importing libraries structs and functions
use hermod::llb::{
    datastr::{DataBase, PkgData},
    handle::handle
};

fn main() {
    let pkg = Arc::new({ PkgData {
        pkgv: String::from(option_env!("CARGO_PKG_VERSION").unwrap_or("0.0.1")),
        apiv: String::from(option_env!("API_VERSION").unwrap_or("0.0.1")),
        deltoken: String::from(option_env!("DEL_TOKEN").unwrap_or("token"))
    }});
    
    let listener = Arc::new(TcpListener::bind("0.0.0.0:2088").expect("[Hermod] Unable to bind to port 2088 on host"));
    
    //Declaration of the KeysVector, it holds all keys to all content of DB, it's set in Arc and RwLock so it can be read by many, modified by one
    let store = Arc::new(DataBase::new());
    store.get_table("_basedb").unwrap().create_record("testkey", "datainside");
    
    println!("Welcome to Hermod\n\n DB v{}, API v{}", pkg.pkgv, pkg.apiv);
    println!(" DEL_TOKEN: {}", pkg.deltoken);
    println!(" Hermod is starting up\n");

    let mut handles = Vec::new();
    let mut counter = 0;

    //The DB is completely ready to receive a connection
    println!("Waiting on port...");
    for stream in listener.incoming() {
        counter += 1;   //Id counter
        let id = counter;
        let pkg_clone = Arc::clone(&pkg);
        let store_clone = Arc::clone(&store);
        handles.push(thread::spawn(move || handle(stream.unwrap(), id, store_clone, pkg_clone)));
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("[Hermod] Shutting down.");
}