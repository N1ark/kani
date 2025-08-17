// Copyright Kani Contributors
// SPDX-License-Identifier: Apache-2.0 OR MIT
// kani-flags: -Zfunction-contracts
//! This test checks that contracts does not havoc
//! [promoted constants](https://github.com/rust-lang/const-eval/blob/master/promotion.md)
//! that represents an enum variant.
//!
//! Related issue: <https://github.com/model-checking/kani/issues/3228>

extern crate kani;
#[derive(PartialEq, Eq, kani::Arbitrary)]
pub enum Foo {
    A,
    B,
}

#[cfg_attr(kani, kani::ensures(|result: &Foo| *result == Foo::A))]
pub fn foo_a() -> Foo {
    Foo::A
}

#[cfg_attr(kani, kani::proof_for_contract(foo_a))]
fn check() {
    let _ = foo_a();
}

#[cfg_attr(kani, kani::proof)]
#[cfg_attr(not(kani), test)]
#[cfg_attr(kani, kani::stub_verified(foo_a))]
fn check_stub() {
    let val = foo_a();
    assert!(val == Foo::A)
}
