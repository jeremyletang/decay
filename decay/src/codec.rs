// Copyright 2016 Jeremy Letang.
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use mime::Mime;
use serde;
use std::collections::BTreeMap as Map;

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Default, Debug)]
pub struct Message {
    headers: Map<String, String>,
    method: String,
    id: u64,
    data: Vec<u8>,
}

pub trait Codec: Sized + Clone + Send + Sync + 'static {
    type Serializer: serde::Serializer;

    fn method(&self, buf: &[u8]) -> Result<String, String>;
    fn mime(&self) -> Mime;
    fn decode<T>(&self, buf: &[u8]) -> Result<T, String>
        where T: serde::Deserialize;
    fn encode<T>(&self, val: &T) -> Result<Vec<u8>, String>
        where T: serde::Serialize;
     
}

pub trait RawCodec {
    fn _method(&self, s: &[u8]) -> Result<String, String>;
    fn _mime(&self) -> Mime;
}

impl<C> RawCodec for C where C: Codec {
    fn _method(&self, buf: &[u8]) -> Result<String, String> {
        self.method(buf)
    }

    fn _mime(&self) -> Mime {
        self.mime()
    }
}

