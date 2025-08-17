// Copyright Kani Contributors
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Empty slices use dangling pointers. Ensure that Kani is ok with that.

#[cfg_attr(kani, kani::proof)]
#[cfg_attr(not(kani), test)]
fn check_empty() {
    let vec = Vec::<f32>::new();
    for float in vec {
        assert!(float.is_nan());
    }
}
