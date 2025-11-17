# bbq2

[![Crates.io](https://img.shields.io/crates/v/bbq2.svg)](https://crates.io/crates/bbq2)
[![Documentation](https://docs.rs/bbq2/badge.svg)](https://docs.rs/bbq2)
[![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](LICENSE)

A lock-free, thread-safe, single-producer single-consumer (SPSC) queue for Rust, based on the BipBuffer algorithm. Now with **sixteen great flavors!** 🍖

This is a reimplementation of [bbqueue](https://github.com/jamesmunns/bbqueue) with improved features and flexibility.

## Features

- **Lock-free & Wait-free**: Contention-free operations for optimal performance
- **Zero-copy**: Efficient data transfer without intermediate buffers
- **`no_std` Compatible**: Perfect for embedded systems and constrained environments
- **Flexible Configuration**: 16 pre-configured "flavors" combining different storage, coordination, and notification strategies
- **Dual Interface Modes**:
  - **Stream Mode**: Continuous byte streams for serial ports, DMA transfers
  - **Framed Mode**: Discrete message boundaries for packets, messages
- **Async/Await Support**: Tokio integration via `maitake-sync`
- **Portable**: Automatically detects and uses platform-appropriate atomic operations

## Quick Start

Add to your `Cargo.toml`:

```toml
[dependencies]
bbq2 = "0.4"
```

### Basic Stream Example

```rust
use bbq2::queue::BBQueue;
use bbq2::traits::{
    coordination::cas::AtomicCoord,
    notifier::blocking::Blocking,
    storage::Inline,
};

// Static allocation with 64-byte buffer
static BBQ: BBQueue<Inline<64>, AtomicCoord, Blocking> = BBQueue::new();

fn main() {
    let prod = BBQ.stream_producer();
    let cons = BBQ.stream_consumer();

    // Producer writes data
    let mut grant = prod.grant_exact(8).unwrap();
    grant.copy_from_slice(&[1, 2, 3, 4, 5, 6, 7, 8]);
    grant.commit(8);

    // Consumer reads data
    let read = cons.read().unwrap();
    assert_eq!(&*read, &[1, 2, 3, 4, 5, 6, 7, 8]);
    read.release(8);
}
```

### Basic Framed Example

```rust
use bbq2::queue::BBQueue;
use bbq2::traits::{
    coordination::cas::AtomicCoord,
    notifier::blocking::Blocking,
    storage::Inline,
};

static BBQ: BBQueue<Inline<64>, AtomicCoord, Blocking> = BBQueue::new();

fn main() {
    let prod = BBQ.framed_producer();
    let cons = BBQ.framed_consumer();

    // Producer writes a frame
    let mut grant = prod.grant(10).unwrap();
    grant[..6].copy_from_slice(&[1, 2, 3, 4, 5, 6]);
    grant.commit(6); // Frame header added automatically

    // Consumer reads the frame
    let frame = cons.read().unwrap();
    assert_eq!(&*frame, &[1, 2, 3, 4, 5, 6]);
    frame.release();
}
```

### Async Example

```rust
use bbq2::queue::BBQueue;
use bbq2::traits::{
    coordination::cas::AtomicCoord,
    notifier::maitake::MaiNotSpsc,
    storage::Inline,
};

static BBQ: BBQueue<Inline<64>, AtomicCoord, MaiNotSpsc> = BBQueue::new();

#[tokio::main]
async fn main() {
    let prod = BBQ.stream_producer();
    let cons = BBQ.stream_consumer();

    tokio::spawn(async move {
        // Wait for data asynchronously
        let read = cons.wait_read().await;
        println!("Received: {:?}", &*read);
        read.release(read.len());
    });

    // Producer writes data
    let mut grant = prod.grant_exact(4).unwrap();
    grant.copy_from_slice(&[10, 20, 30, 40]);
    grant.commit(4);
}
```

## The Sixteen Flavors

BBQ2 provides 16 pre-configured type aliases (in the `nicknames` module) combining:

- **Storage**: `Inline` (compile-time) vs `Heap` (runtime-sized)
- **Coordination**: `Atomic` (CAS) vs `CriticalSection` (fallback)
- **Notifier**: `Blocking` (polling) vs `Async` (tokio)
- **Arc**: Wrapped in `Arc` for easy sharing vs bare references

| Flavor | Storage | Coordination | Notifier | Arc | Origin |
|--------|---------|--------------|----------|-----|---------|
| **Jerk** | Inline | Critical Section | Blocking | No | Jamaica |
| **Asado** | Inline | Critical Section | Blocking | Yes | Argentina |
| **Memphis** | Inline | Critical Section | Async | No | USA |
| **Carolina** | Inline | Critical Section | Async | Yes | USA |
| **Churrasco** | Inline | Atomic | Blocking | No | Brazil |
| **Barbacoa** | Inline | Atomic | Blocking | Yes | Mexico |
| **Texas** | Inline | Atomic | Async | No | USA |
| **KansasCity** | Inline | Atomic | Async | Yes | USA |
| **Braai** | Heap | Critical Section | Blocking | No | South Africa |
| **Kebab** | Heap | Critical Section | Blocking | Yes | Türkiye |
| **SiuMei** | Heap | Critical Section | Async | No | Hong Kong |
| **Satay** | Heap | Critical Section | Async | Yes | SE Asia |
| **YakiNiku** | Heap | Atomic | Blocking | No | Japan |
| **GogiGui** | Heap | Atomic | Blocking | Yes | South Korea |
| **Tandoori** | Heap | Atomic | Async | No | India |
| **Lechon** | Heap | Atomic | Async | Yes | Philippines |

### Using a Flavor

```rust
use bbq2::nicknames::Texas; // Inline storage, Atomic coordination, Async notifier

static BBQ: Texas<1024> = Texas::new();
```

## Architecture

### Storage Traits

- **`Inline<N>`**: Fixed-size buffer allocated at compile time (supports `const` initialization)
- **`BoxedSlice`**: Heap-allocated buffer with runtime-determined size

### Coordination Traits

- **`AtomicCoord`**: Lock-free coordination using compare-and-swap atomics (default on supported platforms)
- **`CsCoord`**: Critical section-based coordination (fallback for platforms without atomic pointer support, like Cortex-M0)

### Notifier Traits

- **`Blocking`**: No-op notifications (use polling)
- **`MaiNotSpsc`**: Async/await support compatible with Tokio

## Interface Modes

### Stream Mode

Multiple producer writes can be coalesced into a single consumer read. Ideal for:
- Serial port data
- DMA transfers
- Continuous data streams

```rust
let prod = BBQ.stream_producer();
let cons = BBQ.stream_consumer();
```

### Framed Mode

Each producer write maintains discrete boundaries with automatic frame headers. Ideal for:
- Network packets
- Message queues
- Protocol implementations

```rust
let prod = BBQ.framed_producer();
let cons = BBQ.framed_consumer();
```

**Note**: Do not mix stream and framed interfaces on the same queue.

## Features

- `default = ["maitake-sync-0_2", "critical-section"]`: Standard configuration
- `std`: Enable standard library support (disabled by default for `no_std`)
- `maitake-sync-0_2`: Enable async/await support via maitake-sync
- `critical-section`: Enable critical section-based coordination
- `disable-cache-padding`: Disable cache padding for reduced memory usage

## Platform Support

BBQ2 automatically detects platform capabilities:

- **Platforms with atomic pointer operations** (e.g., ARM Cortex-M3+, x86, x86_64): Uses `AtomicCoord` for lock-free operation
- **Platforms without atomic pointers** (e.g., ARM Cortex-M0): Falls back to `CsCoord` with critical sections

## Use Cases

- **Embedded Systems**: Real-time data acquisition, sensor processing
- **High-Performance Systems**: Zero-copy inter-thread communication
- **Async Applications**: Tokio-based services and applications
- **Protocol Implementation**: Packet processing, message framing
- **Hardware Interfacing**: UART, SPI, I2C, DMA operations

## Safety

BBQ2 uses unsafe code internally for performance-critical operations, but provides a safe API. The implementation has been:
- Extensively tested with unit tests
- Verified with Miri for undefined behavior
- Designed for single-producer single-consumer scenarios only

## License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contributing

Contributions are welcome! Please see [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

## Acknowledgments

This is a reimplementation of the original [bbqueue](https://github.com/jamesmunns/bbqueue) by James Munns.

## Resources

- [Documentation](https://docs.rs/bbq2/)
- [Repository](https://github.com/jamesmunns/bbq2)
- [Crates.io](https://crates.io/crates/bbq2)
- [BipBuffer Algorithm](https://www.codeproject.com/Articles/3479/The-Bip-Buffer-The-Circular-Buffer-with-a-Twist)
