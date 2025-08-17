// Copyright Kani Contributors
// SPDX-License-Identifier: Apache-2.0 OR MIT
// kani-verify-fail

#![feature(never_type)]

// Transmutes an inhabited ZST into a uninhabited ZST
//
// Handled as a special case of transmute (non returning intrinsic) that
// compiles but crashes at runtime, similar to calling `std::intrinsic::abort`
#[cfg_attr(kani, kani::proof)]
#[cfg_attr(not(kani), test)]
fn main() {
    unsafe { std::mem::transmute::<(), !>(()) };
}
