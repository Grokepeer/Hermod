//Importing standard libraries
use std::{
    str,
    time::Instant,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    sync::{Arc, RwLock, Mutex}
};

use super::datastr::{
    DataBase,
    DataTable,
    KeyData
};

struct

pub fn handle(mut stream: TcpStream, store: Arc<DataBase>) {
    let timestart = Instant::now();
    let mut req = String::from("");  //Vector containing the complete HTTP request (groups together all buffers)
    let mut cl = 0; //Content-Length
    let mut key = String::from("");    //Data Key
    let mut deltoken = String::from("");  //Destruction token
    let mut httpheader = String::from("");  //First HTTP header (GET /path HTTP/1.1)
    let mut reqbody = String::from("");
    let mut clcheck: bool = false; //Content-Length has-been-acquired check
    let dt = String::from("test");

    let mut buffer = BufReader::new(&stream);

    loop {   //The handle reads data by buffers of 500 bytes
        let mut line = String::from("");
        let linelen = buffer.read_line(&mut line).unwrap();
        println!("Line: {}, {}", line, buffer.capacity());

        if linelen == 2 {
            break;
        }
    }

    let mut body: Vec<_> = vec![];
    buffer.read_until(70, &mut body);

    println!("{:?}", String::from_utf8(body).unwrap());
    println!("{httpheader}");

    let bodystring: String;
    let reshead;
    let resbody;
    (reshead, resbody) = match httpheader.as_str() {
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

    stream.write_all(response.as_bytes()).unwrap();
}