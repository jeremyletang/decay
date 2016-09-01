// Copyright 2016 Jeremy Letang.
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![feature(plugin_registrar, rustc_private)]

extern crate decay_codegen;
extern crate rustc_plugin;

#[plugin_registrar]
#[doc(hidden)]
pub fn plugin_registrar(reg: &mut rustc_plugin::Registry) {
    decay_codegen::register(reg);
}
