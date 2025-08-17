// Copyright Kani Contributors
// SPDX-License-Identifier: Apache-2.0 OR MIT
const FIFO_SIZE: usize = 2;
#[cfg_attr(kani, kani::proof)]
#[cfg_attr(not(kani), test)]
fn main() {
    let len: usize = kani::any();
    if !(len <= FIFO_SIZE) {
        return;
    }
    let _buf1: Vec<u8> = vec![0u8; len]; //< this works
    let elt: u8 = kani::any();
    let _buf2: Vec<u8> = vec![elt; len]; //< this fails: "memset destination region writeable"
    let idx: usize = kani::any();
    if idx < len {
        assert!(_buf1[idx] == 0u8);
        assert!(_buf2[idx] == elt);
    }
}

#[cfg_attr(kani, kani::proof)]
#[cfg_attr(not(kani), test)]
fn minimal1() {
    let v: Vec<i8> = vec![kani::any(); 0];
}

#[cfg_attr(kani, kani::proof)]
#[cfg_attr(not(kani), test)]
fn minimal2() {
    let v: Vec<i8> = vec![5; 0];
}

#[cfg_attr(kani, kani::proof)]
#[cfg_attr(not(kani), test)]
fn vec3772() {
    let value: u8 = 1; /* set to zero and it passes */
    let count: u16 = kani::any();
    let vector: Vec<u8> = vec![value; count as usize];
}
