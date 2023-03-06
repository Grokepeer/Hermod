//Importing standard libraries
use std::{
    str,
    time::Instant,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    sync::{Arc, RwLock, Mutex}
};

use super::{
    datastr::{
        DataBase,
        DataTable,
        KeyData
    },
};

pub fn handle(mut stream: String, store: Arc<DataBase>) {
    let timestart = Instant::now();
    let dt = String::from("test");
    println!("Test: {:.2?}", timestart.elapsed());

    let timelapse = Instant::now();
    println!("HTTP req reading done: {:.2?}", timestart.elapsed());

    // println!("Status: {}\nHeaders: {:?}\nBody: {}", httpreq.status, httpreq.headers, httpreq.body);

    // println!("{:?}", String::from_utf8(body).unwrap());
    // println!("{httpheader}");

    let bodystring: String;
    let reshead;
    let resbody;
    (reshead, resbody) = match "" {
        "GET /get HTTP/1.1" => {
            ("200 OK", store.get_table("_basedb").unwrap().get_record("_base").unwrap().data.read().unwrap().to_string())
        }

        _ => {
            println!("Unrecognized path");
            ("400 Bad Request", "Unrecognized path".to_string())
        }
    };
    
    let (status, res_content) = ("HTTP/1.1 ".to_owned() + reshead, resbody.to_string());
    let length = res_content.len();
    let response = format!("{status}\r\nContent-Length: {length}\r\n\r\n{res_content}");

    println!("Query chronometer: {:.2?}", timestart.elapsed());

    // stream.write_all(response.as_bytes()).unwrap();
}