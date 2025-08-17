// Copyright Kani Contributors
// SPDX-License-Identifier: Apache-2.0 OR MIT

#[cfg_attr(kani, kani::proof)]
#[cfg_attr(not(kani), test)]
#[cfg_attr(kani, kani::unwind(10))]
fn main() {
    let mut a: u32 = kani::any();

    if a < 1024 {
        loop {
            a = a / 2;

            if a == 0 {
                break;
            }
        }

        assert!(a == 0);
    }
}
