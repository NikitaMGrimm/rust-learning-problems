use std::{
    sync::{mpsc, Arc, Mutex},
    thread::{self},
};

impl Drop for ThreadPool {
    fn drop(&mut self) {
        drop(self.sender.take());
        let workers = &mut self.workers;
        for worker in workers {
            println!("Worker {id} shutting down.", id = worker.id);
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
        let thread = Some(thread::spawn(move || loop {
            let job = receiver.lock().unwrap().recv();
            match job {
                Ok(job) => {
                    println!("Worker {id} has received a job!");
                    job(); 
                }
                Err(_) => {
                    println!("Worker {id} disconnected.");
                    break;
                }
            }
        }));
        Worker { id, thread }
    }
}

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Job>>,
}

type Job = Box<dyn FnOnce() + Send + 'static>;

impl ThreadPool {
    /// Create a threadpool with `num` Workers
    ///
    /// # Panics
    ///
    /// `new` will panic if `num` is 0
    pub fn new(num: usize) -> ThreadPool {
        assert!(num > 0);

        let (tx, rx) = mpsc::channel();
        let rx = Arc::new(Mutex::new(rx));
        let mut workers = Vec::with_capacity(num);
        for id in 0..num {
            workers.push(Worker::new(id, Arc::clone(&rx)));
        }

        ThreadPool {
            workers,
            sender: Some(tx),
        }
    }
    pub fn execute<T>(&self, f: T)
    where
        T: FnOnce() + Send + 'static,
    {
        let job: Job = Box::new(f);
        self.sender.as_ref().unwrap().send(job).unwrap();
    }
}
