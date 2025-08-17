// Copyright Kani Contributors
// SPDX-License-Identifier: Apache-2.0 OR MIT

use std::fs;
use std::path::PathBuf;
#[cfg_attr(kani, kani::proof)]
#[cfg_attr(not(kani), test)]
#[cfg_attr(kani, kani::unwind(3))]
fn main() {
    let buf = PathBuf::new();
    let _x = fs::remove_file(buf);
}
