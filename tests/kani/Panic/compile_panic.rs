// Copyright Kani Contributors
// SPDX-License-Identifier: Apache-2.0 OR MIT
// kani-check-fail

//! This test checks that Kani fails in compilation due to a panic evaluated at
//! compile time

const fn my_const_fn(x: i32) -> i32 {
    if x > 0 { x - 1 } else { panic!("x is negative") }
}

#[cfg_attr(kani, kani::proof)]
#[cfg_attr(not(kani), test)]
pub fn check_something() {
    const _X: i32 = my_const_fn(-3);
}
