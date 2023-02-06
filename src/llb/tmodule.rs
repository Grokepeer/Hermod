use std::{
    thread,
    sync::{mpsc, Arc, Mutex, RwLock}
};

pub struct KeyData {
    pub key: String,
    pub pair: String,
}

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<JobPacket>>,
}

pub struct JobPacket {
    job: Box<dyn FnOnce() + Send + 'static>,
    store: Arc<RwLock<Vec<KeyData>>>,
}

impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel();

        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        ThreadPool { 
            workers, 
            sender: Some(sender),
        }
    }

    pub fn execute<F>(&self, f: F, kstore: Arc<RwLock<Vec<KeyData>>>)
    where
        F: FnOnce() + Send + 'static,
    {
        let jobpacket = { JobPacket {
            job: Box::new(f),
            store: kstore,
        }};

        match self.sender.as_ref() {
            Some(refer) => {
                match refer.send(jobpacket) {
                    Ok(_) => println!("Done"),
                    Err(_) => println!("Err")
                }
            },
            None => println!("err")
        }
    }
}

struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<JobPacket>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            match receiver.lock() {
                Ok(jp) => { 
                    let packet = jp.recv();
                    let message = match packet {
                        Ok(packet) => {
                            packet
                        }
                        Err(_) => {
                            continue
                        }
                    };

                    println!("First Store entry: {}", message.store.read().unwrap()[0].key);
                    (message.job)();
                },
                Err(_) => { 
                    // println!("[Hermod] Thread {id} got a faulty JobPacket");
                }
            };
        });

        Worker { 
            id, 
            thread: Some(thread), 
        }
    }
}

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