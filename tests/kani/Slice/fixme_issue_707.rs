// Copyright Kani Contributors
// SPDX-License-Identifier: Apache-2.0 OR MIT

#[cfg_attr(kani, kani::proof)]
#[cfg_attr(not(kani), test)]
fn main() {
    let [x, y @ .., z] = [1, 2, 3, 4];
    assert!(x == 1);
    assert!(y[0] == 2);
    assert!(y[1] == 3);
    assert!(z == 4);
}
