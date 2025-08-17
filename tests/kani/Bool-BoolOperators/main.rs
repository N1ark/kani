// Copyright Kani Contributors
// SPDX-License-Identifier: Apache-2.0 OR MIT
#[cfg_attr(kani, kani::proof)]
#[cfg_attr(not(kani), test)]
fn main() {
    assert!(true);
    assert!(true || false);
    assert!(!false);

    let a = true;
    let b = false;
    let c = a || b;
    let d = c && a;
    assert!(d && true);
    assert!(!b && d);

    let e: bool = kani::any();
    assert!(e || !e);
}
