/// A mutithreaded Web server implementation using a thread pool created by using
/// channels, Mutex and Arc.
use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;

// This is a type alias for a trait object that holds the type of closure that execute receives.
type Job = Box<dyn FnOnce() + Send + 'static>;

// This enum represents the 2 types of messages that can be sent to a thread
// Either a Job wrapped in NewJob() or a Terminate message
pub enum Message {
    NewJob(Job),
    Terminate,
}

// Here we need to wrap up the closure as  Box<dyn...> ,
// since its size is not known at compile time so we must store a pointer(Box) to such DST(dynamically sized) objects.
pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Message>, // The channel receives `Message` values
}

pub struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>, // We use option here intead of directly keeping the thread handle
                                            // since the handle will be replaced by None when the threadpool is droped
}

// After we create a threadpool we get the sender and receiver ends of the channel
// We now create workers and pass them a Arc<Mutex<mpsc::Receiver<Message>>>, that is
// we pass a copy of the receiving end wrapped in Arc(to allow multiple mutable owners accross threads)
// and wrapped in mutex(yo ensure only one thread can have access to it and avoid race conditions)
// A worker will wait for jobs in the channel using the shared receiving end of the channel
impl Worker {
    pub fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Worker {
        // Spawn a new worker thread and create an infinite loop !
        let thread = thread::spawn(move || loop {
            // Check for new job in channel by using the receiving end of the channel
            // We must first lock the receiving end since its a mutex
            // `recv()` blocks the thread and waits for the channel untill a message is recieved.
            let message = receiver.lock().unwrap().recv().unwrap();

            match message {
                Message::NewJob(job) => {
                    println!("Worker {} got a job; executing.", id);

                    // Here job is a Box(Job) or Box(Box<dyn FnOnce() + Send + 'static>)),
                    // automatic deref gets us the closure wrapped inside
                    // Executes the colsure by:
                    job();
                }

                // When a terminate message is received the thread breaks out of its infinite loop and thus terminates
                // (join is called from thread pools Drop trait implementation)
                Message::Terminate => {
                    println!("Worker {} was told to terminate.", id);

                    break;
                }
            }

            // At this point after the taks(closure) is done again loop in the infinite loop waiting for the next message
        });

        Worker {
            id,
            thread: Some(thread) // Return the thread handle wrapped in Option::Some(handle)
        }
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
        self.sender.send(Message::NewJob(job)).unwrap();
    }
}

// Implementing thee Drop trait for our Threadpool struct
// This means the drop() method will be automatically called whenever the threadpool object goes out of scope
impl Drop for ThreadPool {
    fn drop(&mut self) {
        println!("Sending terminate message to all workers.");

        // Send a N number terminate messages down the channel of the thread pool
        // where N = number of workers(threads)
        for _ in &self.workers {
            self.sender.send(Message::Terminate).unwrap();
        }

        println!("Shutting down all workers.");

        // For each worker in the thread pool call thread.join()
        // and thus wait for all threads to finish
        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);

            // join() method on a join handle waits for the associated thread to finish.
            // However join requires ownership of the thread handle, thus we use Option::take()
            // which takes the value out of the option, leaving a None in its place.
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}
