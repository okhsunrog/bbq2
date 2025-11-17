//! Sixteen Flavors example
//!
//! This example demonstrates using the pre-configured "flavor" type aliases
//! for different combinations of storage, coordination, and notification.

use bbq2::nicknames::{Churrasco, Texas, YakiNiku};

fn main() {
    println!("BBQ2 Sixteen Flavors Example");
    println!("============================\n");

    // Churrasco: Inline storage, Atomic coordination, Blocking notifier
    println!("1. Churrasco (Brazil) - Inline + Atomic + Blocking");
    static CHURRASCO: Churrasco<128> = Churrasco::new();
    {
        let prod = CHURRASCO.stream_producer();
        let cons = CHURRASCO.stream_consumer();

        let mut grant = prod.grant_exact(7).unwrap();
        grant.copy_from_slice(b"Brazil!");
        grant.commit(7);

        let read = cons.read().unwrap();
        println!("   Received: {}", String::from_utf8_lossy(&*read));
        read.release(7);
    }

    // Texas: Inline storage, Atomic coordination, Async notifier
    println!("\n2. Texas (USA) - Inline + Atomic + Async");
    println!("   (Would require tokio runtime for async operations)");
    static TEXAS: Texas<128> = Texas::new();
    {
        let prod = TEXAS.stream_producer();
        let cons = TEXAS.stream_consumer();

        let mut grant = prod.grant_exact(6).unwrap();
        grant.copy_from_slice(b"Texas!");
        grant.commit(6);

        // In a real async context, you would use wait_read().await
        let read = cons.read().unwrap();
        println!("   Received: {}", String::from_utf8_lossy(&*read));
        read.release(6);
    }

    // YakiNiku: Heap storage, Atomic coordination, Blocking notifier
    println!("\n3. YakiNiku (Japan) - Heap + Atomic + Blocking");
    use bbq2::traits::storage::BoxedSlice;
    let storage = BoxedSlice::new(128);
    let yakiniku = YakiNiku::new_with_storage(storage);
    {
        let prod = yakiniku.stream_producer();
        let cons = yakiniku.stream_consumer();

        let mut grant = prod.grant_exact(6).unwrap();
        grant.copy_from_slice(b"Japan!");
        grant.commit(6);

        let read = cons.read().unwrap();
        println!("   Received: {}", String::from_utf8_lossy(&*read));
        read.release(6);
    }

    println!("\n✓ All flavors work great!");
    println!("\nAvailable flavors:");
    println!("  - Jerk (Jamaica), Asado (Argentina), Memphis (USA), Carolina (USA)");
    println!("  - Churrasco (Brazil), Barbacoa (Mexico), Texas (USA), KansasCity (USA)");
    println!("  - Braai (South Africa), Kebab (Türkiye), SiuMei (Hong Kong), Satay (SE Asia)");
    println!("  - YakiNiku (Japan), GogiGui (South Korea), Tandoori (India), Lechon (Philippines)");
}
