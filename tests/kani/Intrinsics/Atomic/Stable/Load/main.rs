// Copyright Kani Contributors
// SPDX-License-Identifier: Apache-2.0 OR MIT

// Check that `atomic_load` and other variants (stable version) return the
// expected result.

use std::sync::atomic::{AtomicBool, Ordering};

#[cfg_attr(kani, kani::proof)]
#[cfg_attr(not(kani), test)]
fn main() {
    // pub fn load(&self, order: Ordering) -> bool
    // Loads a value from the bool.
    // load takes an Ordering argument which describes the memory ordering
    // of this operation.
    let a = AtomicBool::new(true);

    // https://doc.rust-lang.org/src/core/sync/atomic.rs.html#2349-2361
    assert!(a.load(Ordering::Acquire) == true);
    assert!(a.load(Ordering::Relaxed) == true);
    assert!(a.load(Ordering::SeqCst) == true);
}
