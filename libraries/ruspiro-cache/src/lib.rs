/***********************************************************************************************************************
 * Copyright (c) 2020 by the authors
 *
 * Author: André Borrmann <pspwizard@gmx.de>
 * License: Apache License 2.0 / MIT
 **********************************************************************************************************************/
#![doc(html_root_url = "https://docs.rs/ruspiro-cache/||VERSION||")]
#![cfg_attr(not(any(test, doctest)), no_std)]
#![feature(core_intrinsics)]
// this crate does only compile to something usefull if targeted on Aarch64
#![cfg(target_arch = "aarch64")]

//! # Cache Maintenance Operations
//!
//! If the caches are active on the Raspberry Pi than there might be specific cache operations needed to clean and
//! invalidate the cache to ensure in cross core and/or ARM core to GPU communications the most recent data is seen.
//!
//! # Usage
//!
//! ```no_run
//! use ruspiro_cache as cache;
//!
//! fn doc() {
//!     cache::clean(); // clean the data cache
//!     cache::invalidate(); // invalidate the data cache
//!     cache::cleaninvalidate(); // clean and invalidate the data cache
//! }
//! ```

mod dcache;
pub use dcache::*;

/// Perform a cache clean operation on the entire data cache
pub fn clean() {
  unsafe { dcache::clean_dcache() }
}

/// Perform a cache invalidate operation on the entire data cache
pub fn invalidate() {
  unsafe { dcache::invalidate_dcache() }
}

/// Perform a cache clean and invalidate operation on the entire data cache
pub fn flush() {
  unsafe { dcache::flush_dcache() }
}
