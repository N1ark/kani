// Copyright Kani Contributors
// SPDX-License-Identifier: Apache-2.0 OR MIT
use std::cmp::Ordering;
pub enum Level {
    Error,
}

#[cfg_attr(kani, kani::proof)]
#[cfg_attr(not(kani), test)]
fn main() {
    let left = Level::Error;
    assert!((left as u8).cmp(&0) == Ordering::Equal);
}
