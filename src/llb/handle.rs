//Importing standard libraries
use std::{
    str,
    io::{Write, Read},
    net::{TcpListener, TcpStream},
    sync::{Arc, RwLock, Mutex}
};

use super::datastr::{
    DataBase,
    DataTable,
    KeyData
};

pub fn handle(mut stream: TcpStream, store: DataBase) {
    let mut req = String::from("");  //Vector containing the complete HTTP request (groups together all buffers)
    let mut cl = 0; //Content-Length
    let mut key = String::from("");    //Data Key
    let mut deltoken = String::from("");  //Destruction token
    let mut httpheader = String::from("");  //First HTTP header (GET /path HTTP/1.1)
    let mut reqbody = String::from("");
    let mut clcheck: bool = false; //Content-Length has-been-acquired check
    let dt = String::from("test");

    let badreq = "HTTP/1.1 400 Bad Request\r\nBad Request";

    loop {   //The handle reads data by buffers of 500 bytes, if the first doesn't contain the required headers it drops the request
        let mut buffer = [0; 50];
        match stream.read(&mut buffer) {    //Reads from the stream the first buffer of requests
            Ok(_) => {
                let reqstring = str::from_utf8(&buffer).unwrap();

                if true {    //If there's no Content-Length defined yet it will search for it in the buffer received
                    let reqlines: Vec<_> = reqstring.lines().collect(); //It slices the headers in lines to read each
                    println!("Fuck {:?}", reqlines);
                    
                    //If the header wasn't already set
                    if httpheader == "" {
                        httpheader = String::from(reqlines[0]);
                    }
                    
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
                        }
                        if line.starts_with("Del-Token") {
                            let tcl: Vec<&str> = line.split(":").collect();
                            deltoken = String::from(tcl[1].trim());
                        }
                    }
                }

                req.push_str(reqstring); //Extends the vector containing the complete request

                let mut bodylen = 0;
                if req.contains("\r\n\r\n") {   //If the request has a body it starts to count the length
                    let mut bodysplit = req.splitn(2, "\r\n\r\n");
                    bodysplit.next().unwrap();
                    let body = bodysplit.next().unwrap();
                    bodylen = body.len();

                    if bodylen >= cl {   //If the body (the request part after the double new line) len() is longer than the declared Content-Length it stops reading
                        reqbody = String::from(body);
                        break
                    }
                }
            }
            Err(_) => {
                stream.write_all(badreq.as_bytes()).unwrap();
                return
            }
        }
    }

    println!("{req}");
    println!("{httpheader}");

    let bodystring: String;
    let reshead;
    let resbody;
    (reshead, resbody) = match httpheader.as_str() {
        "GET /get HTTP/1.1" => {
            match "" {
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
            match "" {
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
            match "" {
                Ok(res) => {
                    if deltoken == dt {
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