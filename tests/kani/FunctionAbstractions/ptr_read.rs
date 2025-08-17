// Copyright Kani Contributors
// SPDX-License-Identifier: Apache-2.0 OR MIT

use std::ptr::read;

#[cfg_attr(kani, kani::proof)]
#[cfg_attr(not(kani), test)]
fn main() {
    let var = 1;
    unsafe {
        assert_eq!(read(&var), var);
    }
}
