// Copyright Kani Contributors
// SPDX-License-Identifier: Apache-2.0 OR MIT
// https://doc.rust-lang.org/std/ffi/enum.c_void.html
#[repr(u8)]
pub enum MyCVoid {
    Unused1,
    Unused2,
}

const MAP_FAILED: *mut MyCVoid = !0 as *mut MyCVoid;

fn mmap() -> *mut MyCVoid {
    0 as *mut MyCVoid
}

#[cfg_attr(kani, kani::proof)]
#[cfg_attr(not(kani), test)]
fn main() {
    let v = mmap();
    assert!(v != MAP_FAILED);
    assert!(v.is_null());
}
