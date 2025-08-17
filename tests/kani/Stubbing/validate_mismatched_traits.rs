// Copyright Kani Contributors
// SPDX-License-Identifier: Apache-2.0 OR MIT
//
// kani-flags: --harness harness -Z stubbing
//
//! This tests that we allow trait mismatches between the stub and the original
//! function/method so long as they do not lead to a trait method call being
//! unresolvable.

fn foo<T>(_x: T) -> bool {
    false
}

trait DoIt {
    fn do_it(&self) -> bool;
}

fn bar<T: DoIt + std::cmp::PartialEq<i32>>(x: T) -> bool {
    x == 42
}

#[cfg_attr(kani, kani::proof)]
#[cfg_attr(not(kani), test)]
#[cfg_attr(kani, kani::stub(foo, bar))]
fn harness() {
    assert!(foo(42));
}
