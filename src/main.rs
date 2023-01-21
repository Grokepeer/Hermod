use std::{
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    thread
};

fn main() {
    println!("[Hermod] Up and running...");

    let listener = TcpListener::bind("0.0.0.0:2088").unwrap();
    let pool = ThreadPool::new(12);

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        });
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let header = buf_reader.lines().next().unwrap().unwrap();

    println!("Request: {:#?}", header);

    let (status, res_content) = match &header[..] {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "Got it"),
        "GET /sleep HTTP/1.1" => ("HTTP/1.1 200 OK", "Sleep"),
        _ => ("HTTP/1.1 404 NOT FOUND", "404")
    };
    
    let length = res_content.len();
    let response = format!("{status}\r\nContent-Length: {length}\r\n\r\n{res_content}");
    stream.write_all(response.as_bytes()).unwrap();
}