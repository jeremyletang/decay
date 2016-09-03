// Copyright 2016 Jeremy Letang.
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use syntax;
use aster::expr::ExprBuilder;
use syntax::ext::build::AstBuilder;
use syntax::ext::base::{Annotatable, ExtCtxt};
use syntax::codemap::Span;
use syntax::ptr::P;
use syntax::ast::{self, Expr, MetaItem, TyKind, ItemKind};
use utils::{extract_generics_from_item, struct_ty, camel_to_snake};

fn str_to_lit(s: &str) -> P<Expr> {
    ExprBuilder::new().str(s)
}

fn make_handler_name(cx: &mut ExtCtxt, ty_kind: &TyKind) -> String {
    let crate_name = cx.ecfg.crate_name.to_string() + ".";
    let mod_path = cx.mod_path_stack
        .iter()
        .fold("".to_string(), |acc, seg| acc + seg + ".");
    let mut ty_name = match ty_kind {
        &TyKind::Path(_, ref p) => {
            p.segments.iter().fold("".to_string(), |acc, seg| {
                acc + &camel_to_snake(syntax::print::pprust::ident_to_string(seg.identifier)) + "."
            })
        }
        _ => unreachable!(),
    };
    ty_name.pop();
    crate_name + &mod_path + &ty_name
}

pub fn expand_derive_handler_name(ecx: &mut ExtCtxt,
                                  sp: Span,
                                  meta_item: &MetaItem,
                                  item: &Annotatable,
                                  push: &mut FnMut(Annotatable)) {
    if let Annotatable::Item(ref item) = *item {
        let generics = match extract_generics_from_item(item) {
            Some(generics) => generics,
            None => {
                ecx.span_err(sp,
                             "`#[derive(HandlerName)]` can only be applied to structs or tuple \
                              structs");
                return;
            }
        };

        let ty = struct_ty(ecx, sp, item.ident, &generics);
        let handler_name = str_to_lit(&*make_handler_name(ecx, &(*ty).node));
        let where_clauses = generics.where_clause.clone();

        let impl_item =
            quote_item!(ecx,
                        impl$generics ::decay::handler::HandlerName for $ty $where_clauses {
                            fn name(&self) -> &str {
                                $handler_name
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
