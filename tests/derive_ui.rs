// Copyright 2018 Guillaume Pinot (@TeXitoi) <texitoi@texitoi.eu>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed

#[cfg(feature = "derive")]
#[rustversion::attr(any(not(stable), before(1.73), since(1.71)), ignore)] // MSRV
#[test]
fn ui() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/derive_ui/*.rs");
}
