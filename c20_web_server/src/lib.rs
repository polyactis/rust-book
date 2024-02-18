use std::{sync::{mpsc, Arc, Mutex}, thread};


pub struct ThreadPool{
    workers: Vec<Worker>,
    // Option<> to wrap sender so that ownership can be taken out sometime later.
    sender: Option<mpsc::Sender<Job>>,
}

impl ThreadPool{
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);
        let (sender, receiver) = mpsc::channel();

        let receiver = Arc::new(Mutex::new(receiver));
        let mut workers = Vec::with_capacity(size);
        for id in 0..size{
            workers.push(Worker::new(id, Arc::clone(&receiver)));

        }
        ThreadPool {workers, sender:Some(sender)}
    }

    pub fn execute<F>(&self, f:F)
    where
        F: FnOnce() + Send + 'static,
        {
            let job = Box::new(f);

            self.sender.as_ref().expect("Fail to unwrap self.sender").send(job).unwrap();
        }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        //drop() can handle Option<T>?
        drop(self.sender.take());
        for worker in &mut self.workers {
            eprintln!("Shutting down work {}", worker.id);
            if let Some(thread) = worker.thread.take(){
                thread.join().unwrap();
            }
        }
    }
}

type Job = Box<dyn FnOnce() + Send + 'static>;

struct Worker {
    id: usize,
    // Option<> to wrap thread so that ownership can be taken out sometime later.
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>> ) -> Worker {
        eprintln!("Worker {id} constructed and spawned a thread that loops forever over a receiver.");
        let thread = thread::spawn(move || loop {
            let mesg = receiver.lock().unwrap().recv();
            match mesg {
                Ok(job) => {
                    eprintln!("Worker {id} got a job and is to execute it.");
                    job();
                }
                Err(err) => {
                    eprintln!("Worker {id} received an error {:?} and is shutting down.", err);
                    break;
                }
            }
        });
        // specify the field name for thread as it is Some(thread), not simply thread.
        Worker {id, thread:Some(thread)}
    }
}