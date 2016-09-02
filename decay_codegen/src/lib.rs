// Copyright 2016 Jeremy Letang.
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![feature(rustc_private, plugin)]
//#![plugin(quasi_macros)]
#![allow(unused_imports, unused_variables)]

extern crate rustc_plugin;

mod utils;

pub fn register(reg: &mut rustc_plugin::Registry) {
 
}
