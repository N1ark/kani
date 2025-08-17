// Copyright Kani Contributors
// SPDX-License-Identifier: Apache-2.0 OR MIT

// Check failure for set len on drop case.

// kani-verify-fail

#[cfg_attr(kani, kani::proof)]
#[cfg_attr(not(kani), test)]
fn main() {
    let mut v: Vec<u32> = Vec::new();
    v.extend(42..=42);
    assert!(v[0] == 41); // Incorrect value
}
