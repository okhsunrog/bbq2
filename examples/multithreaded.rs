//! Multi-threaded example
//!
//! This example demonstrates using bbq2 with Arc for cross-thread communication.

use bbq2::queue::ArcBBQueue;
use bbq2::traits::{
    coordination::cas::AtomicCoord, notifier::blocking::Blocking, storage::Inline,
};
use std::thread;
use std::time::Duration;

fn main() {
    println!("BBQ2 Multi-threaded Example");
    println!("===========================\n");

    // Create an Arc-wrapped queue for sharing between threads
    let queue: ArcBBQueue<Inline<256>, AtomicCoord, Blocking> = ArcBBQueue::new();

    let prod = queue.stream_producer();
    let cons = queue.stream_consumer();

    // Spawn producer thread
    let producer_thread = thread::spawn(move || {
        for i in 0..5 {
            println!("Producer: Sending message {}...", i);

            let message = format!("Message {}", i);
            let bytes = message.as_bytes();

            let mut grant = prod.grant_exact(bytes.len()).unwrap();
            grant.copy_from_slice(bytes);
            grant.commit(bytes.len());

            thread::sleep(Duration::from_millis(100));
        }
        println!("Producer: Done sending messages");
    });

    // Spawn consumer thread
    let consumer_thread = thread::spawn(move || {
        let mut received = 0;

        while received < 5 {
            if let Ok(read) = cons.read() {
                let message = String::from_utf8_lossy(&*read);
                println!("Consumer: Received '{}'", message);

                let len = read.len();
                read.release(len);
                received += 1;
            } else {
                // Wait a bit if no data available
                thread::sleep(Duration::from_millis(10));
            }
        }

        println!("Consumer: Done receiving messages");
    });

    // Wait for both threads to complete
    producer_thread.join().unwrap();
    consumer_thread.join().unwrap();

    println!("\n✓ Example completed successfully!");
}
