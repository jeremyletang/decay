// Copyright 2016 Jeremy Letang.
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![feature(plugin, custom_derive)]
#![plugin(serde_macros, decay_macros)]
#![allow(unused_variables)]

extern crate decay;
extern crate serde;

use serde::{Serializer, Deserializer};
use decay::codec::Codec;
use decay::context::Context;
use decay::handler::{Handler, HandlerName, HandlerCodecs};
use decay::mime::Mime;
use decay::fn_handler_wrapper::FnHandlerWrapper;
use decay::json_codec::JsonCodec;
use decay::service::Service;

#[derive(Default, Serialize, Deserialize, Debug)]
pub struct PersonRequest {
    pub yolo: String,
    pub thug: i32,
}
#[derive(Default, Serialize, Deserialize, Debug)]
pub struct PersonResponse {
    pub ok: bool,
    pub error: String,
}

pub struct PersonHandler;

impl Handler<PersonRequest, PersonResponse> for PersonHandler {
    fn handle(&self, ctx: &Context, req: PersonRequest) -> PersonResponse {
        println!("from PersonHandler::handle {:?}", req);
        return PersonResponse {
            ok: false,
            error: "YES BRO'".into(),
        };
    }
}

impl HandlerName for PersonHandler {
    fn name(&self) -> &str {
        "personhandler"
    }
}

impl HandlerCodecs<PersonRequest, PersonResponse> for PersonHandler {
    fn codecs(&self) -> Vec<Mime> {
        vec![JsonCodec{}.mime()]
    }

    fn encode(&self, res: PersonResponse, mime: &Mime) -> Result<Vec<u8>, String> {
        let json_codec = JsonCodec {};
        if *mime == json_codec.mime() {
            json_codec.encode(&res)
        } else {
            Err("".into())
        }
    }
    
    fn decode(&self, buf: &[u8], mime: &Mime) -> Result<PersonRequest, String> {
        let json_codec = JsonCodec {};
        if *mime == json_codec.mime() {
            json_codec.decode(buf)
        } else {
            Err("".into())
        }
    }

}

#[derive(Default, Serialize, Deserialize)]
pub struct UserRequest;
#[derive(Default, Serialize, Deserialize)]
pub struct UserResponse;

fn user(ctx: &Context, _: UserRequest) -> UserResponse {
    Default::default()
}

fn main() {
    let mut service = Service::new("yolo", "http://127.0.0.1:1492");
    service.use_codec(JsonCodec {})
        .use_handler(FnHandlerWrapper { f: user, ..Default::default() })
        .use_handler(PersonHandler {});
    println!("Hello, world!");
}
