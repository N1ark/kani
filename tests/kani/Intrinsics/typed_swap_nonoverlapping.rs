// Copyright Kani Contributors
// SPDX-License-Identifier: Apache-2.0 OR MIT

// Check that `typed_swap_nonoverlapping` yields the expected results.
// https://doc.rust-lang.org/nightly/std/intrinsics/fn.typed_swap_nonoverlapping.html

#![feature(core_intrinsics)]
#![allow(internal_features)]

#[cfg_attr(kani, kani::proof)]
#[cfg_attr(not(kani), test)]
fn test_typed_swap_nonoverlapping_u32() {
    let mut a: u32 = kani::any();
    let a_before = a;
    let mut b: u32 = kani::any();
    let b_before = b;
    unsafe {
        std::intrinsics::typed_swap_nonoverlapping(&mut a, &mut b);
    }
    assert!(b == a_before);
    assert!(a == b_before);
}

#[cfg_attr(kani, kani::proof)]
#[cfg_attr(not(kani), test)]
pub fn check_swap_unit() {
    let mut x: () = kani::any();
    let mut y: () = kani::any();
    std::mem::swap(&mut x, &mut y)
}
