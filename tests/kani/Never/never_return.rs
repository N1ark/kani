// Copyright Kani Contributors
// SPDX-License-Identifier: Apache-2.0 OR MIT
// kani-verify-fail

#![feature(never_type)]

/// Test using the never type
pub fn err() -> ! {
    panic!("EXPECTED FAIL: Function should always fail");
}

// Give an empty main to make rustc happy.
#[cfg_attr(kani, kani::proof)]
#[cfg_attr(not(kani), test)]
fn main() {
    let var = kani::any::<i32>();
    if var > 0 {
        err();
    }
}
