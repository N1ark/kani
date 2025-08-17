// Copyright Kani Contributors
// SPDX-License-Identifier: Apache-2.0 OR MIT

#![feature(ptr_metadata)]

struct S {
    x: i32,
}

trait T {}

impl T for S {}

#[cfg_attr(kani, kani::proof)]
#[cfg_attr(not(kani), test)]
fn ptr_metadata() {
    assert_eq!(std::ptr::metadata("foo"), 3_usize);

    let s = S { x: 42 };
    let p: &dyn T = &s;
    assert_eq!(std::ptr::metadata(p).size_of(), 4_usize);

    let c: char = 'c';
    assert_eq!(std::ptr::metadata(&c), ());
}
