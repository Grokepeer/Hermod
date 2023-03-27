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
        apiv: String::from(option_env!("CARGO__VERSION").unwrap_or("0.0.1")),
        deltoken: String::from(option_env!("Del_Token").unwrap_or("token"))
    }});

    let mut vec: Vec<Arc<[u8; 30]>> = Vec::new();
    vec.push(Arc::new([1; 30]));
    vec.push(Arc::new([2; 30]));
    println!("Vec1: {:p},\nVec2: {:p}\n\n", &vec[0], &vec[1]);

    let mut vec: Vec<[u8; 30]> = Vec::new();
    vec.push([1; 30]);
    vec.push([2; 30]);
    println!("Vec1: {:p},\nVec2: {:p}", &vec[0], &vec[1]);
    
    let listener = Arc::new(TcpListener::bind("0.0.0.0:2088").expect("[Hermod] Unable to bind to port 2088 on host"));
    
    //Declaration of the KeysVector, it holds all keys to all content of DB, it's set in Arc and RwLock so it can be read by many, modified by one
    let store = Arc::new(DataBase::new());
    store.get_table("_basedb").unwrap().create_record("testkey", "datainside");
    
    println!("[Hermod] Hermod v{}, API v{}", pkg.pkgv, pkg.apiv);
    println!("[Hermod] Del_Token: {}", pkg.deltoken);
    println!("[Hermod] Hermod is starting up...");

    let mut handles = Vec::new();
    let mut counter = 0;
    for stream in listener.incoming() {
        counter += 1;
        let id = counter;
        let pkg_clone = Arc::clone(&pkg);
        let store_clone = Arc::clone(&store);
        handles.push(thread::spawn(move || handle(stream.unwrap(), id, store_clone, pkg_clone)));
        break;
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("[Hermod] Shutting down.");
}