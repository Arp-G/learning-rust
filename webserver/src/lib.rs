/// A mutithreaded Web server implementation using a thread pool created by using
/// channels, Mutex and Arc.
use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;

// Type alias
// This is a type alias for a trait object that holds the type of closure that execute receives.
type Job = Box<dyn FnOnce() + Send + 'static>;

// Here we need to wrap up the closure as  Box<dyn...> ,
// since its size is not known at compile time so we must store a pointer(Box) to such DST(dynamically sized) objects.
pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>
}

pub struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}

// After we create a threadpool we get the sender and receiver ends of the channel
// We now create workers and pass them a Arc<Mutex<mpsc::Receiver<Job>>>, that is
// we pass a copy of the receiving end wrapped in Arc(to allow multiple mutable owners accross threads)
// and wrapped in mutex(yo ensure only one thread can have access to it and avoid race conditions)
// A worker will wait for jobs in the channel using the shared receiving end of the channel
impl Worker {
    pub fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        // Spawn a new worker thread
        let thread = thread::spawn(move || loop {
            // Check for new job in channel by using the receiving end of the channel
            // We must first lock the receiving end since its a mutex
            // `recv()` blocks the thread and waits for the channel untill a message is recieved.
            // Here job is a Box(Job) or Box(Box<dyn FnOnce() + Send + 'static>)), automatic deref gets us the closure wrapped inside
            let job = receiver.lock().unwrap().recv().unwrap();

            println!("Worker {} got a job; executing.", id);

            // Execute the closure
            job();
        });

        Worker { id, thread }
    }
}

impl ThreadPool {
    /// Create a new ThreadPool.
    ///
    /// The size is the number of threads in the pool.
    ///
    /// # Panics
    ///
    /// The `new` function will panic if the size is zero.
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel();

        // The receiver end of the channel will be shared by multiple threads
        // that will be looping and asking for jobs on the channel.
        // Thus, The Arc type will let multiple workers own the receiver,
        // and Mutex will ensure that only one worker gets a job from the receiver at a time.
        let receiver = Arc::new(Mutex::new(receiver));

        // Unlike Vec::new resizes itself as elements are inserted.
        // Vec::with_capacity is usefull(more efficient) when the size of the vector is known and can be preallocated.
        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            // For each new worker, we clone the Arc to bump the reference count so the workers can share ownership of the receiving end.
            workers.push(Worker::new(id, Arc::clone(&receiver)))
        }

        ThreadPool { workers, sender }
    }

    // The closure we pass to execute will be further passed to spawn methdo
    // so execute() should try to use the same argument type accepted by spawn.
    // FnOnce is the trait we want to use because the thread for running a request will only execute that request’s closure one time
    // we need Send to transfer the closure from one thread to another and lifetime 'static because we don’t know how long
    // the thread will take to execute.
    // execute(), uses the sender end of the channel to place a Job in the channel which will then be picked up by any waiting workers
    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        // The sender for the channel is like `mpsc::Sender<Job>`
        // and Job is a type alias for `Box<dyn FnOnce() + Send + 'static>`
        // So, we wrap up the closure `f` in a Box pointer
        let job = Box::new(f);

        // Send that job down the sending end of the channel
        // This job will be picked up by one of the free workers which have a reference to receiving end of the channel
        self.sender.send(job).unwrap();
    }
}
