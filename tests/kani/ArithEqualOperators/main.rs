// Copyright Kani Contributors
// SPDX-License-Identifier: Apache-2.0 OR MIT
#[cfg_attr(kani, kani::proof)]
#[cfg_attr(not(kani), test)]
fn main() {
    let mut a: u32 = kani::any();
    a /= 2;
    let mut b: u32 = kani::any();
    b /= 2;
    let c = b;
    b += a;

    let d = a;

    assert!(b > a || a == 0 || c == 0);

    b -= a;

    assert!(c == b);

    a *= 2;

    assert!(a > d || d == 0);
}
