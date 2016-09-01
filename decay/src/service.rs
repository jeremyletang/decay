// Copyright 2016 Jeremy Letang.
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use codec::{Codec, RawCodec};
use handler::{Handler, RawHandler};
use mime::Mime;
use serde;

pub struct Service {
    name: String,
    addr: String,
    handlers: Vec<Box<RawHandler>>,
    codecs: Vec<Box<RawCodec>>,
}

impl Service {
    pub fn new<Name: Into<String>, Addr: Into<String>>(name: Name, addr: Addr) -> Service {
        return Service {
            name: name.into(),
            addr: addr.into(),
            handlers: vec![],
            codecs: vec![],
        };
    }

    pub fn name(&self) -> &str {
        &*self.name
    }

    pub fn addr(&self) -> &str {
        &self.addr
    }
    
    pub fn use_codec<C: 'static>(&mut self, codec: C) -> &mut Service
        where C: Codec
    {
        self.codecs.push(Box::new(codec) as Box<RawCodec>);
        self
    }

    pub fn use_handler<H: 'static, Req: 'static, Res: 'static>(&mut self,
                                                               handler: H)
                                                               -> &mut Service
        where H: Handler<Req, Res>,
              Req: serde::Deserialize + serde::Serialize,
              Res: serde::Deserialize + serde::Serialize
    {
        let box_handler: Box<Handler<Req, Res> + 'static> = Box::new(handler);
        self.handlers.push(Box::new(box_handler) as Box<RawHandler>);
        self
    }
    
    pub fn has_method(&self, method: &str) -> bool {
        self.handlers.iter().any(|ref h| h._name() == method)
    }
    
    pub fn has_codec_support(&self, mime: Mime) -> bool {
        self.codecs.iter().any(|ref c| c._mime() == mime)
    }
    
    pub fn start(self) {}
}
