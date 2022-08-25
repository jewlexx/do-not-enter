# RusPiRo cache maintenance crate

This crates provides several cach maintenance functions that helps clearing or invalidating the cache of the Raspberry Pi.
Especially when it comes to cross core and core to GPU communications (like mailbox calls) the cache need to be cleared/invalidated
to ensure access to the most recent values stores in the memory.

![CI](https://github.com/RusPiRo/ruspiro-cache/workflows/CI/badge.svg?branch=development)
[![Latest Version](https://img.shields.io/crates/v/ruspiro-cache.svg)](https://crates.io/crates/ruspiro-cache)
[![Documentation](https://docs.rs/ruspiro-cache/badge.svg)](https://docs.rs/ruspiro-cache)
[![License](https://img.shields.io/crates/l/ruspiro-cache.svg)](https://github.com/RusPiRo/ruspiro-cache#license)

## Usage

To use the crate just add the following dependency to your ``Cargo.toml`` file:

```toml
[dependencies]
ruspiro-cache = "||VERSION||"
```

Once done the access to the cache maintenance functions is available like so:

```rust
use ruspiro-cache as cache;

fn demo() {
    cache::clean(); // clean the data cache
    cache::invalidate(); // invalidate the data cache
    cache::flush(); // clean and invalidate the data cache
}
```

## License

Licensed under Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0) or MIT ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)) at your choice.
