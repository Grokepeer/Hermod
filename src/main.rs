use std::net::TcpListener;

fn main() {
    println!("[Hermod] Up and running...");

    let listener = TcpListener::bind("0.0.0.0:2088").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        println!("Connection established!");
    }
}