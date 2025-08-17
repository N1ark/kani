// Copyright Kani Contributors
// SPDX-License-Identifier: Apache-2.0 OR MIT

use std::ptr::write;

#[cfg_attr(kani, kani::proof)]
#[cfg_attr(not(kani), test)]
fn main() {
    let mut var = 1;
    unsafe {
        write(&mut var, 10);
    }
    assert_eq!(var, 10);
}
