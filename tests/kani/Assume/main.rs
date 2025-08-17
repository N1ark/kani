// Copyright Kani Contributors
// SPDX-License-Identifier: Apache-2.0 OR MIT

#[cfg_attr(kani, kani::proof)]
#[cfg_attr(not(kani), test)]
fn main() {
    let i: i32 = kani::any();
    kani::assume(i < 10);
    assert!(i < 20);
}

#[cfg_attr(kani, kani::proof)]
#[cfg_attr(not(kani), test)]
fn verify_any_where() {
    // Only single digit values are legal
    let i: i32 = kani::any_where(|x| *x < 10);
    assert!(i < 20);
}
