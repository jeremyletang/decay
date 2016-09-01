// Copyright 2016 Jeremy Letang.
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use serde;

pub trait FnHandler<Req, Res>
    where Req: serde::Deserialize + serde::Serialize,
          Res: serde::Deserialize + serde::Serialize {
    fn handle(&self, ctx: &Context, req: Req) -> Res;
}

impl<F, Req, Res> FnHandler<Req, Res> for F
    where Req: serde::Deserialize + serde::Serialize,
          Res: serde::Deserialize + serde::Serialize,
          F: Fn(&Context, Req) -> Res
{
    fn handle(&self, ctx: &Context, req: Req) -> Res {
        self.call((ctx, req))
    }
}

