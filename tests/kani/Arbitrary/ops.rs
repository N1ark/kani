// Copyright Kani Contributors
// SPDX-License-Identifier: Apache-2.0 OR MIT
//
// Check that users can generate range structures

extern crate kani;

use std::ops::{Bound, Range, RangeFrom, RangeInclusive, RangeTo, RangeToInclusive};

#[cfg_attr(kani, kani::proof)]
#[cfg_attr(not(kani), test)]
fn bound() {
    let elem: Wrapper<Bound<u8>> = kani::any();
    match elem.0 {
        Bound::Included(elem) => {
            assert!(elem < 100);
        }
        Bound::Excluded(elem) => {
            assert!(elem < 100);
        }
        Bound::Unbounded => {}
    }
}

#[cfg_attr(kani, kani::proof)]
#[cfg_attr(not(kani), test)]
fn range() {
    let elem: Wrapper<Range<u8>> = kani::any();
    assert!(elem.0.start < 100);
    assert!(elem.0.end < 100);
}

#[cfg_attr(kani, kani::proof)]
#[cfg_attr(not(kani), test)]
fn range_from() {
    let elem: Wrapper<RangeFrom<u8>> = kani::any();
    assert!(elem.0.start < 100);
}

#[cfg_attr(kani, kani::proof)]
#[cfg_attr(not(kani), test)]
fn range_inclusive() {
    let elem: Wrapper<RangeInclusive<u8>> = kani::any();
    assert!(*elem.0.start() < 100);
    assert!(*elem.0.end() < 100);
}

#[cfg_attr(kani, kani::proof)]
#[cfg_attr(not(kani), test)]
fn range_to() {
    let elem: Wrapper<RangeTo<u8>> = kani::any();
    assert!(elem.0.end < 100);
}

#[cfg_attr(kani, kani::proof)]
#[cfg_attr(not(kani), test)]
fn range_to_inclusive() {
    let elem: Wrapper<RangeToInclusive<u8>> = kani::any();
    assert!(elem.0.end < 100);
}

struct Wrapper<T>(T);

impl kani::Arbitrary for Wrapper<Bound<u8>> {
    fn any() -> Self {
        let val = kani::any();
        match val {
            Bound::Included(elem) => {
                kani::assume(elem < 100);
            }
            Bound::Excluded(elem) => {
                kani::assume(elem < 100);
            }
            Bound::Unbounded => {}
        }
        Self(val)
    }
}

impl kani::Arbitrary for Wrapper<Range<u8>> {
    fn any() -> Self {
        let val = kani::any()..kani::any();
        kani::assume(val.start < 100);
        kani::assume(val.end < 100);
        Self(val)
    }
}

impl kani::Arbitrary for Wrapper<RangeFrom<u8>> {
    fn any() -> Self {
        let val = kani::any()..;
        kani::assume(val.start < 100);
        Self(val)
    }
}

impl kani::Arbitrary for Wrapper<RangeInclusive<u8>> {
    fn any() -> Self {
        let val = kani::any()..=kani::any();
        kani::assume(*val.start() < 100);
        kani::assume(*val.end() < 100);
        Self(val)
    }
}

impl kani::Arbitrary for Wrapper<RangeTo<u8>> {
    fn any() -> Self {
        let val = ..kani::any();
        kani::assume(val.end < 100);
        Self(val)
    }
}

impl kani::Arbitrary for Wrapper<RangeToInclusive<u8>> {
    fn any() -> Self {
        let val = ..=kani::any();
        kani::assume(val.end < 100);
        Self(val)
    }
}
