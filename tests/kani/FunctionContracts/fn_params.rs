// Copyright Kani Contributors
// SPDX-License-Identifier: Apache-2.0 OR MIT
//! Checks that function contracts work with different type of parameter expressions:
//! Source: <https://doc.rust-lang.org/reference/items/functions.html>
//!
//! Note: See `receiver_contracts` for receiver parameters.
// kani-flags: -Zfunction-contracts

extern crate kani;
use std::convert::TryFrom;

/// Dummy structure to check different patterns in contract.
#[derive(Copy, Clone, PartialEq, Eq, kani::Arbitrary)]
struct MyStruct {
    c: char,
    u: u32,
}

/// Add contracts to ensure that all parameters are representing the same pair (char, u32).
#[cfg_attr(kani, kani::requires(val.u == second))]
#[cfg_attr(kani, kani::requires(val.u == tup_u))]
#[cfg_attr(kani, kani::requires(Ok(val.c) == char::try_from(first)))]
#[cfg_attr(kani, kani::requires(val.c == tup_c))]
pub fn odd_parameters_eq(
    [first, second]: [u32; 2],
    (tup_c, tup_u): (char, u32),
    val @ MyStruct { c: val_c, u }: MyStruct,
) {
    assert_eq!(tup_c, char::try_from(first).unwrap());
    assert_eq!(tup_c, val_c);

    assert_eq!(tup_u, second);
    assert_eq!(tup_u, u);
    assert_eq!(val, MyStruct { c: val_c, u });
}

/// Similar to the function above, but with one requirement missing.
#[cfg_attr(kani, kani::requires(val.u == second))]
#[cfg_attr(kani, kani::requires(val.u == tup_u))]
#[cfg_attr(kani, kani::requires(Ok(val.c) == char::try_from(first)))]
// MISSING: #[cfg_attr(kani, kani::requires(val.c == tup_c))]
pub fn odd_parameters_eq_wrong(
    [first, second]: [u32; 2],
    (tup_c, tup_u): (char, u32),
    val @ MyStruct { c: val_c, u }: MyStruct,
) {
    assert_eq!(tup_c, char::try_from(first).unwrap());
    assert_eq!(tup_c, val_c);

    assert_eq!(tup_u, second);
    assert_eq!(tup_u, u);
    assert_eq!(val, MyStruct { c: val_c, u });
}

mod verify {
    use super::*;
    use kani::Arbitrary;

    #[cfg_attr(kani, kani::proof_for_contract(odd_parameters_eq))]
    fn check_params() {
        odd_parameters_eq(kani::any(), kani::any(), kani::any())
    }

    #[cfg_attr(kani, kani::should_panic)]
    #[cfg_attr(kani, kani::proof_for_contract(odd_parameters_eq_wrong))]
    fn check_params_wrong() {
        odd_parameters_eq_wrong(kani::any(), kani::any(), kani::any())
    }
}
