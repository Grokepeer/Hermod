//Importing standard libraries
use std::{
    thread,
    sync::{mpsc, Arc, Mutex}
};

//Data structure used in the KeysVector, points to another space in the heap that contains all data paired with the key in a String
pub struct KeyData {
    pub key: String,
    pub pair: Mutex<String>,
}

//ThreadPool structure that is used by the HTTP Server to send request onto workers, also contains mpsc channel informations for the workers
pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Job>>,
}

//The Job type is used to send the necessary functions to the workers
type Job = Box<dyn FnOnce() + Send + 'static>;

impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);  //ThreadPool size has to be bigger than 0

        let (sender, receiver) = mpsc::channel();   //Creates a sender and receiver from mpsc
        let receiver = Arc::new(Mutex::new(receiver));  //Wraps the receiver in Arc and Mutex so it can be operated by different Workers

        let mut workers = Vec::with_capacity(size); //Creates a vector that stores all worker instances

        //Start up the workers
        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        ThreadPool { 
            workers, 
            sender: Some(sender),
        }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        match self.sender.as_ref() {    //Gets the mpsc channel to send the job request to workers pool
            Some(refer) => {
                match refer.send(Box::new(f)) {   //Sends the boxed job function to a worker
                    Ok(_) => println!("[Hermod] Job sent to worker"),
                    Err(_) => println!("[Hermod] The job couldn't be sent to the worker pool")
                }
            },
            None => println!("[Hermod] Couldn't get mpsc sender channel")
        }
    }
}

//This struct contains data for workers in the ThreadPool
struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || loop {   //Create a Thread for the worker to work in
            match receiver.lock() { //Gets the lock onto the receiver channel to read from it
                Ok(receiver_lock) => { 
                    let packet = match receiver_lock.recv() { //Unwraps the packet that is received from the receiver channel
                        Ok(packet) => {
                            packet
                        }
                        Err(_) => {
                            continue    //If the packet is invalid it skips the loop
                        }
                    };
                    drop(receiver_lock);    //Drop the lock on the receiver so other thread can lock onto it

                    packet();    //Execute the request job
                },
                Err(_) => { 
                    println!("[Hermod] Thread {id} couldn't get a lock on the receiver");
                }
            };
        });

        Worker { 
            id, 
            thread: Some(thread), 
        }
    }
}

//TODO ThreadPool Drop function to stop the DB
impl Drop for ThreadPool {
    fn drop(&mut self) {
        drop(self.sender.take());

        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);

            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}