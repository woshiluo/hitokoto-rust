use std::sync::{mpsc, Arc, Mutex};

type Work = Box<dyn FnOnce() + Send + 'static>;

enum WorkerMessage {
    NewWork(Work),
    Terminate,
}

struct Worker {
    thread: Option<std::thread::JoinHandle<()>>,
}

pub struct WorkerPool {
    sender: mpsc::Sender<WorkerMessage>,
    threads: Vec<Worker>,
}

impl Worker {
    /// Start a new worker
    fn new(id: usize, recvier: Arc<Mutex<mpsc::Receiver<WorkerMessage>>>) -> Worker {
        let thread = std::thread::spawn(move || loop {
            let message = recvier.lock().unwrap().recv().unwrap();
            match message {
                WorkerMessage::NewWork(work) => {
                    log::info!("Worker {} get a new work", id);
                    work();
                }
                WorkerMessage::Terminate => {
                    log::info!("Worker {} terminate", id);
                    return;
                }
            }
        });
        Worker {
            thread: Some(thread),
        }
    }
}

impl WorkerPool {
    /// Return a workerpool with `size` workers.
    pub fn new(size: usize) -> WorkerPool {
        assert!(size > 0);

        let mut threads = Vec::new();
        let (sender, recv) = mpsc::channel();
        let recv = Arc::new(Mutex::new(recv));
        for id in 0..size {
            let recv = recv.clone();
            threads.push(Worker::new(id, recv));
        }

        WorkerPool { sender, threads }
    }
    /// Add a fn to queue.
    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let work = Box::new(f);
        self.sender.send(WorkerMessage::NewWork(work)).unwrap();
    }
}

impl Drop for WorkerPool {
    fn drop(&mut self) {
        for _ in &mut self.threads {
            self.sender.send(WorkerMessage::Terminate).unwrap();
        }
        for worker in &mut self.threads {
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}
