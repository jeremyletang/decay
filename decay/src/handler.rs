// Copyright 2016 Jeremy Letang.
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use codec::{RawCodec, Codec};
use context::Context;
use mime::Mime;
use serde;
use serde_json;

pub trait Handler<Req, Res>: HandlerName + HandlerCodecs<Req, Res>
    where Req: serde::Deserialize + serde::Serialize + Default,
          Res: serde::Deserialize + serde::Serialize
{
    fn handle(&self, ctx: &Context, req: Req) -> Res;
}

pub trait HandlerName {
    fn name(&self) -> &str;
}

pub trait HandlerCodecs<Req, Res>
    where Req: serde::Deserialize + serde::Serialize + Default,
          Res: serde::Deserialize + serde::Serialize{
    fn codecs(&self) -> Vec<Mime>;
    fn encode(&self, res: Res, mime: &Mime) -> Result<Vec<u8>, String>;
    fn decode(&self, buf: &[u8], mime: &Mime) -> Result<Req, String>;
}

pub trait RawHandler {
    fn _handle(&self, ctx: &Context, buf: &[u8], mime: &Mime) -> Vec<u8>;
    fn _name(&self) -> &str;
}

impl<Req, Res> RawHandler for Box<Handler<Req, Res>>
    where Req: serde::Deserialize + serde::Serialize + Default,
          Res: serde::Deserialize + serde::Serialize
{
    fn _handle(&self, ctx: &Context, buf: &[u8], mime: &Mime) -> Vec<u8> {
        let r: Req = match self.decode(buf, mime) {
            Ok(g) => g,
            Err(_) => panic!("error"),
        };
        let res: Res = self.handle(ctx, r);
        self.encode(res, mime).ok().unwrap()
    }

    fn _name(&self) -> &str {
        self.name()
    }
}
