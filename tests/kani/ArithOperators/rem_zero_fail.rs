// Copyright Kani Contributors
// SPDX-License-Identifier: Apache-2.0 OR MIT
// kani-verify-fail

// Check that remainder triggers overflow checks.
// Covers the case where `b == 0`.

#[cfg_attr(kani, kani::proof)]
#[cfg_attr(not(kani), test)]
fn main() {
    let a: i8 = kani::any();
    let b: i8 = kani::any();
    kani::assume(b == 0);
    let _ = a % b;
}
