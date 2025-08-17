// Copyright Kani Contributors
// SPDX-License-Identifier: Apache-2.0 OR MIT
// kani-verify-fail

// Check that `assume` fails if the condition is false (undefined behavior)
#![feature(core_intrinsics)]

#[cfg_attr(kani, kani::proof)]
#[cfg_attr(not(kani), test)]
fn main() {
    unsafe { core::intrinsics::assume(false) };
}
