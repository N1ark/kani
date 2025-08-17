// Copyright Kani Contributors
// SPDX-License-Identifier: Apache-2.0 OR MIT

#[cfg_attr(kani, kani::proof)]
#[cfg_attr(not(kani), test)]
#[cfg_attr(kani, kani::unwind(11))]
fn main() {
    let mut a: u32 = kani::any();

    if a < 1024 {
        while a > 0 {
            a = a / 2;
        }

        assert!(a == 0);
    }
}
