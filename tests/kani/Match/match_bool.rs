// Copyright Kani Contributors
// SPDX-License-Identifier: Apache-2.0 OR MIT

#[cfg_attr(kani, kani::proof)]
#[cfg_attr(not(kani), test)]
pub fn match_bool() {
    let arg: bool = kani::any();
    let var = match arg {
        true => !arg,
        _ => arg,
    };

    let var2 = match arg {
        _ => false,
    };

    assert!(var == var2);

    let mut i = 0;

    match arg {
        true => i = 1,
        false => i = 2,
        _ => i = 3,
    }

    assert!(i == 1 || i == 2);
}
