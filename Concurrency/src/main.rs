use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::Duration;
use rand::Rng;

fn ass3() {
// Message to be sent to the workers
enum Message {
    NewJob(Job),
    Terminate,
}

// Job type is a boxed closure that can be sent across threads
type Job = Box<dyn FnOnce() + Send + 'static>;

// ThreadPool struct
struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Message>,
}

impl ThreadPool {
    // Create a new ThreadPool with the specified size
    fn new(size: usize) -> ThreadPool {
        assert!(size > 0);
        
        // Create a channel for sending jobs
        let (sender, receiver) = mpsc::channel();
        
        let receiver = Arc::new(Mutex::new(receiver));
        
        // Create and store workers
        let mut workers = Vec::with_capacity(size);
        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }
        
        // Return the ThreadPool
        ThreadPool { workers, sender }
    }
    
    // Execute a job in the thread pool
    fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        // Create a job from the closure and send it to a worker
        let job = Box::new(f);
        self.sender.send(Message::NewJob(job)).unwrap();
    }
}

// Clean up resources when ThreadPool is dropped
impl Drop for ThreadPool {
    fn drop(&mut self) {
        // Send a terminate message to each worker
        for _ in &self.workers {
            self.sender.send(Message::Terminate).unwrap();
        }
        
        // Join each worker's thread
        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}
// Worker struct represents a thread that can process jobs
struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    // Create a new worker with the specified ID
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Worker {
        // Create a thread that loops and receives jobs from the channel
        let thread = thread::spawn(move || {
            loop {
                let message = receiver.lock().unwrap().recv().unwrap();
                match message {
                    Message::NewJob(job) => {
                        println!("Worker {} got a job; executing.", id);
                        job();
                    }
                    Message::Terminate => {
                        println!("Worker {} was told to terminate.", id);
                        break;
                    }
                }
            }
        });
 // Return the Worker
        Worker {
            id,
            thread: Some(thread),
        }
    }
}
// Create a new thread pool with 4 workers
    let pool = ThreadPool::new(4);
    // Submit 10 tasks to the pool
    for i in 1..=10 {
        pool.execute(move || {
            println!("Processing task {}", i);
            thread::sleep(std::time::Duration::from_millis(500));
            println!("Completed task {}", i);
        });
    }
    println!("Main thread waiting for tasks to complete...");
    // ThreadPool will be dropped when it goes out of scope, triggering the cleanup
}

fn ass4() {
    
// Define a special value that will signal termination
const TERMINATION_SIGNAL: i32 = -1;

// Number of items to produce
const ITEM_COUNT: usize = 20;
const NUM_PRODUCERS: usize = 2;
const NUM_CONSUMERS: usize = 3;

// Create a channel for sending numbers
let (tx, rx) = mpsc::channel();

// Share the receiver among consumer threads using Arc<Mutex>
let rx = Arc::new(Mutex::new(rx));

// Create 2 producer threads
let mut producer_handles = vec![];
for id in 0..NUM_PRODUCERS {
    let tx_clone = tx.clone();
    let items_per_producer = ITEM_COUNT / NUM_PRODUCERS;
    let handle = thread::spawn(move || {
        producer(id, tx_clone, items_per_producer);
    });
    producer_handles.push(handle);
}
    
    // Create 3 consumer threads
    let mut consumer_handles = vec![];
    for id in 0..NUM_CONSUMERS {
        let rx_clone = Arc::clone(&rx);
        let handle = thread::spawn(move || {
            consumer(id, rx_clone);
        });
        consumer_handles.push(handle);
    }
    
    // Wait for all producer threads to finish
    for handle in producer_handles {
        handle.join().unwrap();
    }
    println!("All producers have finished!");
    
    // After producers have sent all their data items, send termination signals
    for _ in 0..NUM_CONSUMERS {
        tx.send(TERMINATION_SIGNAL).unwrap();
    }
    
    // Wait for all consumer threads to finish
    for handle in consumer_handles {
        handle.join().unwrap();
    }
    
    println!("All items have been produced and consumed!");

// Producer function
fn producer(id: usize, tx: mpsc::Sender<i32>, item_count: usize) {
    let mut rng = rand::thread_rng();
    
    for i in 0..item_count {
        // Generate a random number between 0 and 100
        let num = rng.gen_range(0..=100);
        println!("Producer {} sending: {}", id, num);
        tx.send(num).unwrap();
        
        // Simulate some work
        thread::sleep(Duration::from_millis(rng.gen_range(50..200)));
    }
    println!("Producer {} finished sending {} items", id, item_count);
}

// Consumer function
fn consumer(id: usize, rx: Arc<Mutex<mpsc::Receiver<i32>>>) {
    let mut items_processed = 0;
    
    loop {
        // Lock the mutex to receive a value
        let value = {
            let receiver = rx.lock().unwrap();
            receiver.recv().unwrap()
        };
        // Check if received value equals TERMINATION_SIGNAL
        if value == TERMINATION_SIGNAL {
            println!("Consumer {} received termination signal, exiting...", id);
            break;
        }
        // Process the value normally
        items_processed += 1;
        println!("Consumer {} processing: {} (items processed: {})", id, value, items_processed);
        
        // Simulate processing time
        thread::sleep(Duration::from_millis(100));
    }
    println!("Consumer {} finished, processed {} items", id, items_processed);
}

}

fn main() {
    println!();
    println!("Assignment 3:");
    println!();
    ass3();
    println!();
    println!("Assignment 4:");
    println!();
    ass4();
    println!();
}