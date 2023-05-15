use std::{thread::{self, JoinHandle}, sync::{mpsc, Arc, Mutex}, println};

// Our thread pool will have workers
pub struct ThreadPool {
  workers: Vec<Worker>,
  sender: mpsc::Sender<Job>
}

type Job = Box<dyn FnOnce() + Send + 'static>;

impl ThreadPool {
  pub fn new(size: usize) -> ThreadPool {
    // thread pool has to have atleast 1 thread
    assert!(size > 0);

    // make a vector for workers
    let mut workers = Vec::with_capacity(size);

    // make a channel to send jobs to workers
    let (sender, receiver) = mpsc::channel();
    let receiver = Arc::new(Mutex::new(receiver));

    // deploy workers
    for id in 0..size {
      workers.push(Worker::new(id, Arc::clone(&receiver)))
    }

    ThreadPool { workers, sender }
  }

  pub fn execute<F>(&self, f: F)
  where
    F: FnOnce() + Send + 'static,
  {
    let job = Box::new(f);
    self.sender.send(job).unwrap();
  }
}

pub struct Worker {
  id: usize,
  thread: thread::JoinHandle<()>,
}

impl Worker {
  fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
    let thread = thread::spawn(move || loop{
      let job = receiver.lock().unwrap().recv().unwrap();
      println!("Worker {} got a job.", id);

      job();
      });
    Worker {id, thread}
  }
}