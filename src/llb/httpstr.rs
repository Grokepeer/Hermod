use std::{
    io::{prelude::*, BufReader},
    net::{TcpStream}
};

pub struct HTTP {
    pub status: String,
    pub headers: Vec<String>,
    pub body: String
}

impl HTTP {
    pub fn new(stream: &TcpStream) -> Result<HTTP, &'static str> {
        let mut buffer = BufReader::new(stream);
        let mut status = String::from("");
        let mut headers = Vec::new();
        let oplist = vec!["GET", "DEL", "SET"];

        {   //Reading the first http request line (GET / HTTP/1.1)
            let mut linebuf = String::from("");
            buffer.read_line(&mut linebuf);
            linebuf = linebuf.replace("\r\n", "");

            let mut httpv = false;
            let mut path = false;
            let mut op = false;
            let mut general = false;
            for pz in linebuf.split(" ") {  //Splits the line to check that all 3 elements are present
                if pz == "HTTP/1.1" {
                    httpv = true;
                } else if pz.starts_with("/") {
                    path = true;
                } else if oplist.contains(&pz) {
                    op = true;
                } else {    //This is to check that the line doesn't contain any extra information that shouldn't be there
                    general = true
                }
                
            }

            if httpv && path && op && !general {    //If the status line is ok
                headers.push(linebuf);
                status = String::from(&headers[0]);
            } else {
                return Err("Invalid request header")
            }
        }

        let mut contentlength = 0;
        loop {  //This loop reads all the request headers until the double new line (/r/n/r/n)
            let mut linebuf = String::from("");
            let linelen = buffer.read_line(&mut linebuf).unwrap();
            linebuf = linebuf.replace("\r\n", "");

            if linebuf.starts_with("Content-Length") {
                let tcl: Vec<&str> = line.split(":").collect();
                contentlength = match tcl[1].trim().parse() {  //Casts CT to u8, if it fails the content length is considered 0
                    Ok(cl) => cl,
                    Err(_) => 0
                };
            }
            
            if linelen <= 2 {   //If there's an empty line
                break;
            } else {
                headers.push(linebuf);
            }
        }

        let mut body = String::from("")
        if contentlength > 0 {
            loop {
                let mut buffer = [0; 500];
                let size = buffer.read(&mut buffer).unwrap();
                body.push_str()
                if size < 500 {
                    break;
                }
            }
        }

        Ok( HTTP {
            status: status,
            headers: headers,
            body: body
        })
    }
}