// Copyright Kani Contributors
// SPDX-License-Identifier: Apache-2.0 OR MIT
//! Test that we can handle passing closure as function pointer.

/// Invoke given function with the given 'input'.
fn invoke(input: usize, f: fn(usize) -> usize) -> usize {
    f(input)
}

#[cfg_attr(kani, kani::proof)]
#[cfg_attr(not(kani), test)]
fn check_closure_ptr() {
    let input = kani::any();
    let output = invoke(input, |x| x);
    assert_eq!(output, input);
}
