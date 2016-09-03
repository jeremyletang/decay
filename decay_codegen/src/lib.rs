// Copyright 2016 Jeremy Letang.
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![feature(rustc_private, plugin)]
#![plugin(quasi_macros)]
#![allow(unused_imports, unused_variables)]

extern crate aster;
extern crate rustc_plugin;
extern crate syntax;
extern crate quasi;

use syntax::feature_gate::AttributeType::CrateLevel;
use syntax::ext::base::MultiDecorator;

mod codecs;
mod handler_name;
mod handler_codecs;
mod utils;

pub fn register(reg: &mut rustc_plugin::Registry) {
    reg.register_attribute("feature(custom_derive)".into(), CrateLevel);
    reg.register_attribute("feature(custom_attribute)".into(), CrateLevel);

    reg.register_syntax_extension(syntax::parse::token::intern("derive_HandlerName"),
                                  MultiDecorator(Box::new(handler_name::expand_derive_handler_name)));

    reg.register_syntax_extension(syntax::parse::token::intern("handler_codecs"),
                                  MultiDecorator(Box::new(handler_codecs::expand_handler_codecs)));
}
