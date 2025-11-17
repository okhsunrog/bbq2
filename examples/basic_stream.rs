//! Basic stream mode example
//!
//! This example demonstrates the basic usage of bbq2 in stream mode,
//! where multiple writes can be coalesced into a single read.

use bbq2::queue::BBQueue;
use bbq2::traits::{
    coordination::cas::AtomicCoord, notifier::blocking::Blocking, storage::Inline,
};

// Static allocation with 256-byte buffer
static BBQ: BBQueue<Inline<256>, AtomicCoord, Blocking> = BBQueue::new();

fn main() {
    println!("BBQ2 Stream Mode Example");
    println!("========================\n");

    let prod = BBQ.stream_producer();
    let cons = BBQ.stream_consumer();

    // Write some data
    println!("Writing data to queue...");
    let mut grant = prod.grant_exact(8).unwrap();
    grant.copy_from_slice(&[1, 2, 3, 4, 5, 6, 7, 8]);
    grant.commit(8);

    // Read the data
    println!("Reading data from queue...");
    let read = cons.read().unwrap();
    println!("Read {} bytes: {:?}", read.len(), &*read);
    read.release(8);

    // Write multiple times
    println!("\nWriting multiple chunks...");
    for i in 0..3 {
        let mut grant = prod.grant_exact(4).unwrap();
        grant.copy_from_slice(&[i * 10, i * 10 + 1, i * 10 + 2, i * 10 + 3]);
        grant.commit(4);
        println!("  Wrote chunk {}: [{}, {}, {}, {}]", i, i * 10, i * 10 + 1, i * 10 + 2, i * 10 + 3);
    }

    // Read all at once (stream mode coalesces reads)
    println!("\nReading coalesced data...");
    let read = cons.read().unwrap();
    println!("Read {} bytes: {:?}", read.len(), &*read);
    read.release(read.len());

    println!("\n✓ Example completed successfully!");
}
