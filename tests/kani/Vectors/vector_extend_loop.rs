// Copyright Kani Contributors
// SPDX-License-Identifier: Apache-2.0 OR MIT

// kani-flags: --default-unwind 3

#[cfg_attr(kani, kani::proof)]
#[cfg_attr(not(kani), test)]
#[cfg_attr(kani, kani::unwind(3))]
fn main() {
    let mut v: Vec<u32> = Vec::new();
    for (start, len) in vec![(0, 1), (1, 2)] {
        v.extend(start..=(start + len - 1));
    }
}
