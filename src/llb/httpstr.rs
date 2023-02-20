use std::{
    io::{
        prelude::*,
        BufReader
    }
};

pub struct HTTP {
    status: String,
    headers: Vec<String>,
    body: String
}

impl HTTP {
    pub fn new<T>(buffer: BufReader<T>) -> HTTP {
        let mut http = HTTP {
            status: String::from(""),
            headers: vec![],
            body: String::from("")
        };

        buffer.read_line(&mut http.status);
        loop {   //The handle reads data by buffers of 500 bytes
            let mut line = String::from("");
            let linelen = buffer.read_line(&mut line).unwrap();
            http.headers.push(line);
    
            if linelen <= 2 {
                break;
            }
        }

        http
    }
}