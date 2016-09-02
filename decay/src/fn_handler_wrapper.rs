// Copyright 2016 Jeremy Letang.
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use context::Context;
use handler::{Handler, HandlerName, HandlerCodecs};
use mime::Mime;
use serde;

pub struct FnHandlerWrapper<Req, Res>
    where Req: serde::Deserialize + serde::Serialize,
          Res: serde::Deserialize + serde::Serialize
{
    pub f: fn(&Context, Req) -> Res,
    pub name: String,
    pub codecs: Vec<Mime>,
}

impl<Req, Res> Default for FnHandlerWrapper<Req, Res>
    where Req: serde::Deserialize + serde::Serialize,
          Res: serde::Deserialize + serde::Serialize
{
    fn default() -> FnHandlerWrapper<Req, Res> {
        FnHandlerWrapper {
            f: {
                fn m<Req_, Res_>(_: &Context, _: Req_) -> Res_ {
                    panic!("need to specify the function at least")
                };
                m
            },
            name: Default::default(),
            codecs: Default::default(),
        }
    }
}


impl<Req, Res> Handler<Req, Res> for FnHandlerWrapper<Req, Res>
    where Req: serde::Deserialize + serde::Serialize + Default,
          Res: serde::Deserialize + serde::Serialize
{
    fn handle(&self, ctx: &Context, req: Req) -> Res {
        (self.f)(ctx, req)
    }
}


impl<Req, Res> HandlerName for FnHandlerWrapper<Req, Res>
    where Req: serde::Deserialize + serde::Serialize,
          Res: serde::Deserialize + serde::Serialize
{
    fn name(&self) -> &str {
        &*self.name
    }
}

impl<Req, Res> HandlerCodecs<Req, Res> for FnHandlerWrapper<Req, Res>
    where Req: serde::Deserialize + serde::Serialize + Default,
          Res: serde::Deserialize + serde::Serialize
{
    fn codecs(&self) -> Vec<Mime> {
        self.codecs.clone()
    }

    fn encode(&self, res: Res, mime: &Mime) -> Result<Vec<u8>, String> {
        Ok(vec![])
    }

    fn decode(&self, buf: &[u8], mime: &Mime) -> Result<Req, String> {
        Ok(Default::default())
    }
}
