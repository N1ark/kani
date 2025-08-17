// Copyright Kani Contributors
// SPDX-License-Identifier: Apache-2.0 OR MIT

#[cfg_attr(kani, kani::proof)]
#[cfg_attr(not(kani), test)]
fn main() {
    let s = String::from("hello");
    let _b = s.into_boxed_str();
}
