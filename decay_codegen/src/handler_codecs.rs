// Copyright 2016 Jeremy Letang.
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use aster::ident::ToIdent;
use codecs;
use syntax;
use syntax::ext::build::AstBuilder;
use syntax::ext::base::{Annotatable, ExtCtxt};
use syntax::codemap::Span;
use syntax::parse::token::str_to_ident;
use syntax::ptr::P;
use syntax::ast::{self, MetaItem, TyKind, ItemKind, Path,
                  PathParameters, PathSegment};
use utils::{extract_generics_from_item, struct_ty};

fn make_codecs_default_path(codec_paths: Vec<Path>) -> Vec<Path> {
    let paths_iter = codec_paths.clone().into_iter();
    codec_paths.into_iter().map(|mut c| {
        c.segments.push(PathSegment{
            identifier: "default".to_ident(),
            parameters: PathParameters::none(),
        });
        c
    }).collect()
}

pub fn expand_handler_codecs(ecx: &mut ExtCtxt,
                             sp: Span,
                             meta_item: &MetaItem,
                             item: &Annotatable,
                             push: &mut FnMut(Annotatable)) {
    let codecs = codecs::extract_codecs_from_meta_item(ecx, meta_item);
    let codecs_default = make_codecs_default_path(codecs).into_iter();

    if let Annotatable::Item(ref item) = *item {
        let mut generics = match extract_generics_from_item(item) {
            Some(generics) => generics,
            None => {
                ecx.span_err(sp,
                             "`#[handler_codecs(...)]` can only be applied to structs or tuple structs");
                return;
            }
        };

        let ty = struct_ty(ecx, sp, item.ident, &generics);

        let mut params = generics.ty_params.into_vec();
        params.push(ecx.typaram(sp, str_to_ident("__REQ"), P::new(), None));
        params.push(ecx.typaram(sp, str_to_ident("__RES"), P::new(), None));
        generics.ty_params = params.into();
        let codecs_default = make_codecs_default_path(codecs).into_iter();

        let impl_item =
            quote_item!(ecx,
                        impl$generics ::decay::handler::HandlerCodecs<__REQ, __RES> for $ty
                            where __REQ: ::serde::Deserialize + ::serde::Serialize + Default,
                                  __RES: ::serde::Serialize + ::serde::Deserialize {
                            fn codecs(&self) -> Vec<Mime> {
                                use ::decay::codec::Codec;
                                vec![$($codecs_default().mime(),)*]
                            }
                            fn encode(&self, req: __RES, mime: &::decay::mime::Mime) -> Result<Vec<u8>, String> {
                                Err("".into())
                            }
                            fn decode(&self, buf: &[u8], mime: &::decay::mime::Mime) -> Result<__REQ, String> {
                                Err("".into())
                            }
                        }
            ).unwrap();

        println!("{}", syntax::print::pprust::item_to_string(&impl_item.clone().unwrap()));
        push(Annotatable::Item(impl_item));
    } else {
        ecx.span_err(meta_item.span,
                     "`derive` may only be applied to enums and structs");
    };
}
