# SigmaOS-linux-ubuntu

> **Ubuntu integration** — Linux distribution persona for SigmaOS.

Part of the [SigmaOS](https://github.com/AaryanSinghChauhan09/SigmaOS) ecosystem.  
[Ubuntu Linux (snap, apt, cloud-ready)](https://ubuntu.com)

## Quickstart

```bash
cargo test    # run deterministic AAA unit tests
cargo check   # verify compilation
```

## Component Role

This crate implements the `ubuntu` subsystem for SigmaOS:
- Fully real Rust implementation (no stubs)
- Fail-fast validation with typed errors
- Deterministic unit tests (Arrange-Act-Assert pattern)
- Single Responsibility, Modular, Zero-Copy where applicable

## API

```rust
use sigmaos_linux_ubuntu::*;
// See src/lib.rs for full public API
```

## Documentation

Full documentation is in the [SigmaOS GitHub Wiki → Linux-Distro-Personas](https://github.com/AaryanSinghChauhan09/SigmaOS/wiki/Linux-Distro-Personas).

## License

GPL-2.0
