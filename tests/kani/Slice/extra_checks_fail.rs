// Copyright Kani Contributors
// SPDX-License-Identifier: Apache-2.0 OR MIT
//
// kani-flags: --extra-pointer-checks
// kani-verify-fail

//! Empty slices use dangling pointers. With extra pointer checks, this test fails due to
//! arithmetic operations using a dangling pointer

#[cfg_attr(kani, kani::proof)]
#[cfg_attr(not(kani), test)]
fn check_empty_fails() {
    let vec = Vec::<f32>::new();
    for float in vec {
        assert!(float.is_nan());
    }
}
