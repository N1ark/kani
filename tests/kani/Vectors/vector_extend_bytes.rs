// Copyright Kani Contributors
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Check that we properly handle `Vec::extend` with a constant byte slice.
//! This used to fail previously (see
//! https://github.com/model-checking/kani/issues/2656).

#[cfg_attr(kani, kani::unwind(4))]
#[cfg_attr(kani, kani::proof)]
#[cfg_attr(not(kani), test)]
fn check_extend_const_byte_slice() {
    const MY_CONSTANT: &[u8] = b"Hi";

    let mut my_vec: Vec<u8> = Vec::new();
    my_vec.extend(MY_CONSTANT);
    assert_eq!(my_vec, [72, 105]);
}
