// Copyright Kani Contributors
// SPDX-License-Identifier: Apache-2.0 OR MIT
// static global variables
static X: bool = false;
static mut Y: bool = false;

#[derive(PartialEq)]
pub enum MyEnum {
    ChoiceA,
    ChoiceB,
    ChoiceC,
}

#[cfg_attr(kani, kani::proof)]
#[cfg_attr(not(kani), test)]
fn main() {
    assert!(!X);
    unsafe {
        Y = true;
        assert!(Y);
    }

    // enum literals will also be codegen'ed as static global variables
    let mut v = MyEnum::ChoiceA;
    assert!(v != MyEnum::ChoiceB);
    v = MyEnum::ChoiceB;
    assert!(v != MyEnum::ChoiceC);
}
