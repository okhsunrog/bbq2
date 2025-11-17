//! Basic framed mode example
//!
//! This example demonstrates the basic usage of bbq2 in framed mode,
//! where each write maintains discrete message boundaries.

use bbq2::queue::BBQueue;
use bbq2::traits::{
    coordination::cas::AtomicCoord, notifier::blocking::Blocking, storage::Inline,
};

// Static allocation with 256-byte buffer
static BBQ: BBQueue<Inline<256>, AtomicCoord, Blocking> = BBQueue::new();

fn main() {
    println!("BBQ2 Framed Mode Example");
    println!("========================\n");

    let prod = BBQ.framed_producer();
    let cons = BBQ.framed_consumer();

    // Write three separate frames
    println!("Writing frames to queue...");

    let mut grant = prod.grant(10).unwrap();
    grant[..6].copy_from_slice(&[1, 2, 3, 4, 5, 6]);
    grant.commit(6);
    println!("  Wrote frame 1: [1, 2, 3, 4, 5, 6]");

    let mut grant = prod.grant(10).unwrap();
    grant[..4].copy_from_slice(&[10, 20, 30, 40]);
    grant.commit(4);
    println!("  Wrote frame 2: [10, 20, 30, 40]");

    let mut grant = prod.grant(10).unwrap();
    grant[..8].copy_from_slice(&[100, 101, 102, 103, 104, 105, 106, 107]);
    grant.commit(8);
    println!("  Wrote frame 3: [100, 101, 102, 103, 104, 105, 106, 107]");

    // Read frames one by one
    println!("\nReading frames from queue...");

    let frame = cons.read().unwrap();
    println!("  Read frame 1 ({} bytes): {:?}", frame.len(), &*frame);
    frame.release();

    let frame = cons.read().unwrap();
    println!("  Read frame 2 ({} bytes): {:?}", frame.len(), &*frame);
    frame.release();

    let frame = cons.read().unwrap();
    println!("  Read frame 3 ({} bytes): {:?}", frame.len(), &*frame);
    frame.release();

    // Queue should now be empty
    assert!(cons.read().is_err());
    println!("\n✓ Example completed successfully!");
}
