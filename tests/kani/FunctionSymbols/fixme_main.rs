// Copyright Kani Contributors
// SPDX-License-Identifier: Apache-2.0 OR MIT

// size_of is not supported yet:
#[cfg_attr(kani, kani::proof)]
#[cfg_attr(not(kani), test)]
fn assert_fndef_zst() {
    assert_eq!(std::mem::size_of_val(&h), 0);
}
