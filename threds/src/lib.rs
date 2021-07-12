use std::{
    sync::{
        mpsc::{channel, Receiver, Sender},
        Arc, Mutex,
    },
    thread,
};

struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
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
    fn new(id: usize, receiver: Arc<Mutex<Receiver<Message>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            let job = receiver.lock().unwrap().recv().unwrap();

            match job {
                Message::NewJob(job) => {
                    println!("worker {} got job, executing", id);
                    job.call();
                }
                Message::Terminate => {
                    break;
                }
            }
        });
        Worker {
            id,
            thread: Some(thread),
        }
    }
}

type Job = Box<dyn FnBox + Send + 'static>;

pub struct Threadpool {
    worker: Vec<Worker>,
    sender: Sender<Message>,
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
        self.sender.send(Message::NewJob(job)).unwrap();
    }
}

enum Message {
    NewJob(Job),
    Terminate,
}

impl Drop for Threadpool {
    fn drop(&mut self) {
        println!("shutting down worker threads");

        //send message to terminate all threads
        for _ in &mut self.worker {
            self.sender.send(Message::Terminate).unwrap();
        }

        //wait for all routine to stop
        for worker in &mut self.worker {
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}
