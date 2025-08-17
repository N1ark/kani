// Copyright Kani Contributors
// SPDX-License-Identifier: Apache-2.0 OR MIT
use std::path::Path;

#[cfg_attr(kani, kani::proof)]
#[cfg_attr(not(kani), test)]
fn main() {
    let path = Path::new("./foo/bar.txt");
}
