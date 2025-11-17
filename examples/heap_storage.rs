//! Heap storage example
//!
//! This example demonstrates using BoxedSlice for runtime-sized buffers
//! instead of compile-time fixed-size buffers.

use bbq2::queue::BBQueue;
use bbq2::traits::{
    coordination::cas::AtomicCoord, notifier::blocking::Blocking, storage::BoxedSlice,
};

fn main() {
    println!("BBQ2 Heap Storage Example");
    println!("=========================\n");

    // Create a runtime-sized buffer
    let buffer_size = 512;
    println!("Creating queue with {} byte heap-allocated buffer", buffer_size);

    let storage = BoxedSlice::new(buffer_size);
    let queue: BBQueue<BoxedSlice, AtomicCoord, Blocking> = BBQueue::new_with_storage(storage);

    let prod = queue.stream_producer();
    let cons = queue.stream_consumer();

    // Write a large message
    let message = "This is a large message stored in a heap-allocated buffer. \
                   Heap storage is useful when the buffer size needs to be \
                   determined at runtime rather than compile time.";

    println!("\nWriting message ({} bytes)...", message.len());
    let mut grant = prod.grant_exact(message.len()).unwrap();
    grant.copy_from_slice(message.as_bytes());
    grant.commit(message.len());

    // Read the message
    println!("Reading message...");
    let read = cons.read().unwrap();
    let received = String::from_utf8_lossy(&*read);
    println!("\nReceived:\n{}", received);

    let len = read.len();
    read.release(len);

    println!("\nExample completed successfully!");
}
