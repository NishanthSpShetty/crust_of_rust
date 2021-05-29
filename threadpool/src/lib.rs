use std::sync::mpsc::{channel, Sender};
use std::sync::{Arc, Mutex};

pub struct ThreadPool {
    handles: Vec<std::thread::JoinHandle<()>>,
    sender: Sender<Box<dyn Fn() + Send>>,
}

impl ThreadPool {
    pub fn new(num_threads: u8) -> Self {
        let (sender, reciever) = channel::<Box<dyn Fn() + Send>>();
        let reciever = Arc::new(Mutex::new(reciever));
        let handles = (0..num_threads)
            .map(|_| {
                let reciever = reciever.clone();
                std::thread::spawn(move || loop {
                    println!(">Start");
                    let work = match reciever.lock().unwrap().recv() {
                        Ok(work) => work,
                        Err(_) => {
                            println!("sender closed, shutting down the threadpool");
                            break;
                        }
                    };

                    work();
                    println!(">Finished");
                })
            })
            .collect();
        Self { handles, sender }
    }

    pub fn execute<T: Fn() + Send + 'static>(&self, work: T) {
        self.sender.send(Box::new(work)).unwrap();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_threadpool() {
        let pool = ThreadPool::new(1);
        pool.execute(|| println!("Hello from thread"));
        pool.execute(|| println!("Hello from thread"));
    }
}
