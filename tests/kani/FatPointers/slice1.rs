// Copyright Kani Contributors
// SPDX-License-Identifier: Apache-2.0 OR MIT

#[cfg_attr(kani, kani::proof)]
#[cfg_attr(not(kani), test)]
fn main() {
    let array = [1, 2, 3, 4, 5, 6];
    let slice: &[u32] = &array;
    assert!(slice[0] == 1);
    assert!(slice[5] == 6);
}
