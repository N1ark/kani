// Copyright Kani Contributors
// SPDX-License-Identifier: Apache-2.0 OR MIT

#[cfg_attr(kani, kani::proof)]
#[cfg_attr(not(kani), test)]
fn main() {
    let slice = &[1, 2, 3][..];
    if let [head, tail @ ..] = slice {
        assert!(head == &slice[0]);
        assert!(tail == &slice[1..]);
    } else {
        unreachable!();
    }
}
