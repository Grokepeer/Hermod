//Importing standard libraries
use std::{
    time::Instant,
    sync::{Arc}
};

use super::{
    datastr::DataBase
};

pub fn handle(command: String, store: Arc<DataBase>) -> u8 {
    let timestart = Instant::now();
    // let dt = String::from("test");
    println!("Test: {:.2?}", timestart.elapsed());

    let timelapse = Instant::now();
    println!("HTTP req reading done: {:.2?}", timelapse.elapsed());

    // println!("Status: {}\nHeaders: {:?}\nBody: {}", httpreq.status, httpreq.headers, httpreq.body);

    // println!("{:?}", String::from_utf8(body).unwrap());
    // println!("{httpheader}");

    if command.starts_with("exit") {
        return 1;
    }

    println!("Query chronometer: {:.2?}", timestart.elapsed());

    return 0;
}