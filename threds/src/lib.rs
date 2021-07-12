use std::{
    sync::{
        mpsc::{channel, Receiver, Sender},
        Arc, Mutex,
    },
    thread,
};

struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}

trait FnBox {
    fn call(self: Box<Self>);
}

impl<F: FnOnce()> FnBox for F {
    fn call(self: Box<F>) {
        (*self)();
    }
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || {
            let job = receiver.lock().unwrap().recv().unwrap();
            println!("worker {} got job, executing", id);
            job.call();
        });
        Worker { id, thread }
    }
}

type Job = Box<dyn FnBox + Send + 'static>;

pub struct Threadpool {
    worker: Vec<Worker>,
    sender: Sender<Job>,
}

impl Threadpool {
    pub fn new(size: usize) -> Threadpool {
        assert!(size > 0);
        let mut worker = Vec::with_capacity(size);
        let (sender, receiver) = channel();

        let mutexed_receiver = Mutex::new(receiver);
        let arcd_receiever = Arc::new(mutexed_receiver);

        for id in 0..size {
            worker.push(Worker::new(id, arcd_receiever.clone()));
        }
        Threadpool { worker, sender }
    }

    //similar to thread::spwan, but we are not interested in value.
    pub fn execute<F>(&self, task: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(task);
        self.sender.send(job).unwrap();
    }
}

impl Drop for Threadpool {
    fn drop(&mut self) {
        println!("shutting down worker threads");

        for worker in &mut self.worker.iter_mut() {}
    }
}
