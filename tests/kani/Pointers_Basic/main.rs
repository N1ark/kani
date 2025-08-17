// Copyright Kani Contributors
// SPDX-License-Identifier: Apache-2.0 OR MIT
#[cfg_attr(kani, kani::proof)]
#[cfg_attr(not(kani), test)]
fn main() {
    let x = 3;
    let y = &x;
    let mut z = *y;

    assert!(z == 3);
}
