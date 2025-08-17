// Copyright Kani Contributors
// SPDX-License-Identifier: Apache-2.0 OR MIT
//! Ensure that we can handle cast and drop of the mutex to a slice

use std::sync::Arc;
use std::sync::Mutex;

#[cfg_attr(kani, kani::proof)]
#[cfg_attr(not(kani), test)]
fn check_drop_slice() {
    let _: Arc<Mutex<[u8]>> = Arc::new(Mutex::new([10, 0]));
}
