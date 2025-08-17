// Copyright Kani Contributors
// SPDX-License-Identifier: Apache-2.0 OR MIT
#[cfg_attr(kani, kani::proof)]
#[cfg_attr(not(kani), test)]
fn main() {
    let a: bool = kani::any();
    let b: bool = kani::any();
    let c = a ^ b;
    assert!((a == b && !c) || (a != b && c));
}
