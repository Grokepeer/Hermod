use std::{
    thread,
    sync::{mpsc, Arc, Mutex}
};

pub struct KeyData<'a> {
    pub key: String,
    pub pair: &'a str
}

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Job>>,
}

type Job = Box<dyn FnOnce() + Send + 'static>;

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

    pub fn execute<F>(&self, f: F, store: &Vec<KeyData>)
    where
        F: FnOnce() + Send + 'static,
    {
        println!("{}", store[0].key);
        let job = Box::new(f);

        match self.sender.as_ref() {
            Some(refer) => {
                match refer.send(job) {
                    Ok(send) => println!("Done"),
                    Err(e) => println!("Err")
                }
            },
            None => println!("err")
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

struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            match receiver.lock() {
                Ok(msg) => { 
                    let message = msg.recv();

                    match message {
                        Ok(job) => {
                            println!("Worker {id} got a job; executing.");
        
                            job();
                        }
                        Err(_) => {
                            println!("Worker {id} disconnected; shutting down.");
        
                            break;
                        }
                    }
                },
                Err(e) => { 
                    //Nothing
                }
            };
        });

        Worker { 
            id, 
            thread: Some(thread), 
        }
    }
}