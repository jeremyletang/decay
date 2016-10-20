// Copyright 2016 Jeremy Letang.
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![feature(plugin, proc_macro)]
#![plugin(serde_derive)]
#![allow(unused_variables)]

#[macro_use]
extern crate log;
extern crate mime as mime_crate;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
extern crate serde;

pub mod mime {
    pub use mime_crate::*;
}

pub mod codec;
pub mod context;
pub mod fn_handler_wrapper;
pub mod handler;
pub mod service;
