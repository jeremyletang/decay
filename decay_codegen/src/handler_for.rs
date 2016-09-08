// Copyright 2016 Jeremy Letang.
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use aster::ident::ToIdent;
use aster::expr::ExprBuilder;
use codecs;
use syntax;
use syntax::ext::build::AstBuilder;
use syntax::ext::base::{Annotatable, ExtCtxt};
use syntax::codemap::Span;
use syntax::parse::token::str_to_ident;
use syntax::ptr::P;
use syntax::ast::{self, MetaItem, TyKind, ItemKind, Path, Item, Ty, Generics, WhereClause,
                  PathParameters, PathSegment, MetaItemKind, NestedMetaItemKind};
use utils::{extract_generics_from_item, struct_ty};

pub fn extract_fn_name_from_meta_item(ecx: &mut ExtCtxt, meta_item: &MetaItem) -> String {
    match &meta_item.node {
        &MetaItemKind::List(_, ref l) => {
            // only use the first item which should be the function name
            match &(l[0].node) {
                &NestedMetaItemKind::MetaItem(ref s) => {
                    match s.node {
                        MetaItemKind::Word(ref w) => w.to_string(),
                        _ => ecx.span_fatal(meta_item.span, "handler_for attribute must be word")
                    }
                },
                _ => ecx.span_fatal(meta_item.span, "handler_for attribute must be word"),
            }
        },
        _ => ecx.span_fatal(meta_item.span, "handler_for attribute must be word"),
    }
}

pub fn make_default_impl_item(
    ecx: &mut ExtCtxt,
    sp: &Span,
    item: &Item,
    ty: &P<Ty>,
    generics: &Generics,
    where_clauses: &WhereClause,
) -> P<Item> {
    let struct_expr = Path {
        span: sp.clone(),
        global: false,
        segments: vec![
            PathSegment{
                identifier: item.ident.clone(),
                parameters: PathParameters::none(),
            },
        ],
    };

    quote_item!(
        ecx,
        impl$generics ::std::default::Default for $ty $where_clauses {
            fn default() -> $ty { $struct_expr(user_handler) }
        }
    ).unwrap()
}

pub fn extract_fn_ty(
    ecx: &mut ExtCtxt,
    sp: &Span,
    item: &ast::Item) {
    match item.node {
        ItemKind::Struct(ref variant, _) => {
            match variant {
                &ast::VariantData::Tuple(ref fields, _) => {
                    if fields.len() != 1 {
                        ecx.span_fatal(*sp, "handler wrapper for function must be a tuple struct which contains only one fn variant");
                    }
                    
                },
                _ => ecx.span_fatal(*sp, "handler wrapper for function must be a tuple struct"),
            }
        },
        _ => ecx.span_fatal(*sp, "handler wrapper for function must be a tuple struct"),
    }
}

pub fn expand_handler_for(
    ecx: &mut ExtCtxt,
    sp: Span,
    meta_item: &MetaItem,
    item: &Annotatable,
    push: &mut FnMut(Annotatable)) {
    let handler_for_fn_name = extract_fn_name_from_meta_item(ecx, meta_item);

    if let Annotatable::Item(ref item) = *item {
        let generics = match extract_generics_from_item(item) {
            Some(generics) => generics,
            None => {
                ecx.span_err(sp, "`#[handler_for(...)]` can only be applied to tuple structs");
                return;
            }
        };
        
        let ty = struct_ty(ecx, sp, item.ident, &generics);
        let where_clauses = generics.where_clause.clone();
        let default_impl_item = make_default_impl_item(ecx, &sp, &item, &ty, &generics, &where_clauses);

        extract_fn_ty(ecx, &sp, item);
        
        let handler_impl_item = quote_item! {
            ecx,
            impl Handler<UserRequest, UserResponse> for UserHandler {
                fn handle(&self, ctx: &Context, req: UserRequest) -> UserResponse {
                    (self.0)(ctx, req)
                }
            }
        }.unwrap();

        println!("{}", syntax::print::pprust::item_to_string(&default_impl_item.clone().unwrap()));
        println!("{}", syntax::print::pprust::item_to_string(&handler_impl_item.clone().unwrap()));
        push(Annotatable::Item(default_impl_item));
        push(Annotatable::Item(handler_impl_item));
    } else {
        ecx.span_err(meta_item.span,
                     "`derive` may only be applied to enums and structs");
    };
}
