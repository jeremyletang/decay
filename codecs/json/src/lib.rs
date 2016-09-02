// Copyright 2016 Jeremy Letang.
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

extern crate decay;
extern crate serde;
extern crate serde_json;

use decay::codec;
use decay::mime::{Mime, TopLevel, SubLevel};
use std::error::Error;

// alias to the codec type, easy to use with compiler plugin
pub type Codec = JsonCodec;

#[derive(Clone)]
pub struct JsonCodec {}

impl codec::Codec for JsonCodec {
    fn method(&self, buf: &[u8]) -> Result<String, String> {
        let message: Result<codec::Message, _> = serde_json::from_slice(buf);
        match message {
            Ok(m) => Ok(m.method),
            Err(e) => Err(e.description().to_string()),
        }
    }

    fn mime(&self) -> Mime {
        return Mime(TopLevel::Application, SubLevel::Json, vec![]);
    }

    fn decode<T>(&self, buf: &[u8]) -> Result<T, String>
        where T: serde::Deserialize
    {
        match serde_json::from_slice(buf) {
            Ok(t) => Ok(t),
            Err(e) => Err(e.description().to_string()),
        }
    }

    fn encode<T>(&self, val: &T) -> Result<Vec<u8>, String>
        where T: serde::Serialize
    {
        match serde_json::to_vec(val) {
            Ok(s) => Ok(s),
            Err(e) => Err(e.description().to_string()),
        }
    }
}


