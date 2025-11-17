//! Async example with Tokio
//!
//! This example demonstrates async/await support using the maitake-sync notifier,
//! which is compatible with Tokio.

use bbq2::queue::BBQueue;
use bbq2::traits::{
    coordination::cas::AtomicCoord, notifier::maitake::MaiNotSpsc, storage::Inline,
};
use std::time::Duration;

// Static allocation with async notifier
static BBQ: BBQueue<Inline<256>, AtomicCoord, MaiNotSpsc> = BBQueue::new();

#[tokio::main]
async fn main() {
    println!("BBQ2 Async/Tokio Example");
    println!("========================\n");

    let prod = BBQ.stream_producer();
    let cons = BBQ.stream_consumer();

    // Spawn a consumer task that waits for data
    let consumer_task = tokio::spawn(async move {
        println!("Consumer: Waiting for data...");

        let read = cons.wait_read().await;
        println!("Consumer: Received {} bytes: {:?}", read.len(), &*read);
        let len = read.len();
        read.release(len);

        println!("Consumer: Waiting for more data...");
        let read = cons.wait_read().await;
        println!("Consumer: Received {} bytes: {:?}", read.len(), &*read);
        let len = read.len();
        read.release(len);
    });

    // Spawn a producer task that writes data with delays
    let producer_task = tokio::spawn(async move {
        println!("Producer: Waiting 1 second...");
        tokio::time::sleep(Duration::from_secs(1)).await;

        println!("Producer: Writing first message...");
        let mut grant = prod.grant_exact(12).unwrap();
        grant.copy_from_slice(b"Hello, BBQ2!");
        grant.commit(12);

        println!("Producer: Waiting 1 second...");
        tokio::time::sleep(Duration::from_secs(1)).await;

        println!("Producer: Writing second message...");
        let mut grant = prod.grant_exact(13).unwrap();
        grant.copy_from_slice(b"Async is fun!");
        grant.commit(13);
    });

    // Wait for both tasks to complete
    let _ = tokio::join!(consumer_task, producer_task);

    println!("\nExample completed successfully!");
}
