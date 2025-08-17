// Copyright Kani Contributors
// SPDX-License-Identifier: Apache-2.0 OR MIT
//
// kani-flags: --harness harness -Z stubbing
//
//! This tests stubbing `std` functions like `std::fs::read`.

fn mock_read<P>(_: P) -> std::io::Result<Vec<u8>> {
    Ok(vec![42])
}

#[cfg_attr(kani, kani::proof)]
#[cfg_attr(not(kani), test)]
#[cfg_attr(kani, kani::stub(std::fs::read, mock_read))]
fn harness() {
    assert_eq!(std::fs::read("ignored").unwrap()[0], 42);
}
