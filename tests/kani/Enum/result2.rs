// Copyright Kani Contributors
// SPDX-License-Identifier: Apache-2.0 OR MIT
#[derive(Debug, PartialEq)]
pub enum Empty {}

#[cfg_attr(kani, kani::proof)]
#[cfg_attr(not(kani), test)]
fn main() {
    let res: Result<Empty, u32> = Err(0);
    if let Err(num) = res {
        num + 1;
    } else {
        3;
    }
}
