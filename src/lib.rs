use std::thread;
use std::sync::{mpsc, Arc, Mutex};
pub struct ThreadPool{
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}

// Being similar to `function pointer`
type Job = Box<dyn FnOnce() + Send + 'static>;

impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        assert!(size >0);

        let (sender, receiver) = mpsc::channel();

        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);
        // The following code is trying to pass `receiver` to multiple `Worker` instance.
        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        ThreadPool {workers, sender}
    }

    // Recall: Take closure that types are three.
    // Fn, FnMut, FnOnce

    // Recall: Trait bound: limitation of implementing type
    // i.e. <T: Display> is a subset of <T> (<T: Display> は <T>の部分集合)
    // pub fn spawn<F, T>(f: F) -> JoinHandle<T>
    //     where
    // F: FnOnce() -> T,
    // F: Send + 'static,
    // T: Send + 'static,

    pub fn execute<F>(&self, f: F)
    where
    F: FnOnce() + Send + 'static, {
        let job = Box::new(f);

        self.sender.send(job).unwrap();
    }
}

struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            let job = receiver.lock().unwrap().recv().unwrap();

            println!("Worker {id} got a job; executing!");

            job();
        });

        Worker { id, thread }
    }
}

