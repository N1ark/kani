// Copyright Kani Contributors
// SPDX-License-Identifier: Apache-2.0 OR MIT

/// This test checks that Kani handles UTF-8-encoded string literals correctly

#[cfg_attr(kani, kani::proof)]
#[cfg_attr(not(kani), test)]
fn check_utf8() {
    let s = "⌚⌛⛪";

    let mut chars = s.chars();
    assert_eq!(chars.next(), Some('⌚'));
    assert_eq!(chars.next(), Some('⌛'));
    assert_eq!(chars.next(), Some('⛪'));
    assert_eq!(chars.next(), None);
}
