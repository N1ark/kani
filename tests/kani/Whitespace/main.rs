// Copyright Kani Contributors
// SPDX-License-Identifier: Apache-2.0 OR MIT

#[cfg_attr(kani, kani::proof)]
#[cfg_attr(not(kani), test)]
#[cfg_attr(kani, kani::unwind(2))]
fn main() {
    let mut iter = "A few words".split_whitespace();
    match iter.next() {
        None => assert!(false),
        Some(x) => assert!(x == "A"),
    }
}
