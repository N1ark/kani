// Copyright Kani Contributors
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! This test checks if Kani can handle multiple harnesses in
//! different directories (first, second, main) that share the same
//! harness name (check). Previously, this caused Kani to crash. See
//! issue #661 for details.

mod first {
    #[cfg_attr(kani, kani::proof)]
    #[cfg_attr(not(kani), test)]
    fn check() {
        assert_eq!(1 + 1, 2);
    }
}

mod second {
    #[cfg_attr(kani, kani::proof)]
    #[cfg_attr(not(kani), test)]
    fn check() {
        assert_eq!(2 + 2, 4);
    }
}

#[cfg_attr(kani, kani::proof)]
#[cfg_attr(not(kani), test)]
pub fn check() {
    assert_eq!(3 + 3, 6);
}
