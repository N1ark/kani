// Copyright Kani Contributors
// SPDX-License-Identifier: Apache-2.0 OR MIT

// Make sure we can handle implicit memcpy on the empty string

fn take_string_ref(s: &str, l: usize) {
    assert!(s.len() == l)
}

#[cfg_attr(kani, kani::proof)]
#[cfg_attr(not(kani), test)]
fn main() {
    take_string_ref(&"x".to_string(), 1);
    take_string_ref(&"".to_string(), 0);
}
