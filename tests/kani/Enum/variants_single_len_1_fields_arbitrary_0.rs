// Copyright Kani Contributors
// SPDX-License-Identifier: Apache-2.0 OR MIT
#[derive(Debug, PartialEq)]
pub enum EnumSingle {
    MySingle,
}

#[cfg_attr(kani, kani::proof)]
#[cfg_attr(not(kani), test)]
fn main() {
    let e = EnumSingle::MySingle;
    assert!(e == EnumSingle::MySingle);
}
