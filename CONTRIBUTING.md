# Contributing to bbq2

Thank you for your interest in contributing to bbq2! This document provides guidelines and information for contributors.

## Code of Conduct

Please be respectful and constructive in all interactions. We aim to maintain a welcoming and inclusive community.

## How to Contribute

### Reporting Bugs

If you find a bug, please open an issue on GitHub with:

- A clear, descriptive title
- Steps to reproduce the issue
- Expected behavior vs. actual behavior
- Your environment (Rust version, target platform, features enabled)
- Minimal code example demonstrating the issue

### Suggesting Enhancements

Enhancement suggestions are welcome! Please open an issue describing:

- The use case or problem you're trying to solve
- Your proposed solution
- Any alternatives you've considered
- Whether you're willing to implement it yourself

### Pull Requests

1. **Fork the repository** and create a branch for your changes
2. **Make your changes** following the coding guidelines below
3. **Add tests** for any new functionality
4. **Run the test suite** to ensure all tests pass
5. **Run Miri** if modifying unsafe code: `./miri.sh`
6. **Update documentation** as needed
7. **Submit a pull request** with a clear description of your changes

## Development Setup

### Prerequisites

- Rust (latest stable)
- Cargo

### Building the Project

```bash
# Standard build
cargo build

# Build with all features
cargo build --all-features

# Build for no_std
cargo build --no-default-features
```

### Running Tests

```bash
# Run all tests
cargo test

# Run tests with all features
cargo test --all-features

# Run specific test
cargo test smoke

# Run with tokio support
cargo test --features maitake-sync-0_2
```

### Running Examples

```bash
# Basic stream mode
cargo run --example basic_stream

# Basic framed mode
cargo run --example basic_framed

# Async/Tokio example
cargo run --example async_tokio --features maitake-sync-0_2

# Multi-threaded example
cargo run --example multithreaded

# Flavors example
cargo run --example flavors

# Heap storage example
cargo run --example heap_storage
```

### Memory Safety Testing with Miri

Miri is used to detect undefined behavior in unsafe code:

```bash
# Run Miri tests
./miri.sh

# Or manually:
cargo +nightly miri test
```

## Coding Guidelines

### Style

- Follow standard Rust style guidelines (use `rustfmt`)
- Run `cargo fmt` before committing
- Run `cargo clippy` and address warnings

### Documentation

- Add doc comments for public APIs
- Include examples in doc comments where helpful
- Update README.md if adding user-facing features
- Add examples to `examples/` directory for significant features

### Unsafe Code

bbq2 uses unsafe code for performance-critical operations. When modifying unsafe code:

- Add clear comments explaining why it's safe
- Ensure proper synchronization and memory ordering
- Test thoroughly with Miri
- Consider all edge cases and race conditions

### Testing

- Write unit tests for new functionality
- Test both success and failure cases
- Test with different configurations (storage types, coordination, notifiers)
- Verify behavior on different platforms if possible

### Commit Messages

- Use clear, descriptive commit messages
- Reference issue numbers when applicable
- Keep commits focused and atomic

## Architecture Overview

### Core Components

1. **Storage Traits** (`traits/storage.rs`)
   - `Inline<N>`: Compile-time fixed-size buffers
   - `BoxedSlice`: Runtime heap-allocated buffers

2. **Coordination Traits** (`traits/coordination/`)
   - `AtomicCoord`: Lock-free CAS-based coordination
   - `CsCoord`: Critical section-based coordination

3. **Notifier Traits** (`traits/notifier/`)
   - `Blocking`: No-op polling notifications
   - `MaiNotSpsc`: Async/await Tokio-compatible notifications

4. **Producer/Consumer Interfaces** (`prod_cons/`)
   - `stream`: Coalesced byte stream interface
   - `framed`: Discrete frame interface with headers

### Adding New Features

When adding features, consider:

- **Backward compatibility**: Avoid breaking existing APIs
- **`no_std` compatibility**: Ensure features work without std
- **Platform support**: Test on different architectures if possible
- **Performance**: Maintain lock-free characteristics where applicable
- **Safety**: Prove correctness of unsafe code

## Feature Flags

Current feature flags:

- `std`: Enable standard library support
- `maitake-sync-0_2`: Enable async/await support
- `critical-section`: Enable critical section coordination
- `disable-cache-padding`: Disable cache padding

When adding new features:
- Add appropriate Cargo.toml entries
- Document the feature in README.md
- Consider interaction with other features

## Release Process

(For maintainers)

1. Update version in Cargo.toml
2. Update CHANGELOG.md (if exists)
3. Run full test suite including Miri
4. Create a git tag: `git tag -a v0.x.y -m "Release v0.x.y"`
5. Push tag: `git push origin v0.x.y`
6. Publish to crates.io: `cargo publish`

## Questions?

If you have questions about contributing, feel free to:

- Open an issue for discussion
- Check existing issues and pull requests
- Review the documentation at https://docs.rs/bbq2/

## License

By contributing to bbq2, you agree that your contributions will be licensed under the same license as the project (MIT OR Apache-2.0).
