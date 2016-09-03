// Copyright 2016 Jeremy Letang.
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use syntax::ast::{self, ItemKind};
use syntax::codemap::Span;
use syntax::ext::build::AstBuilder;
use syntax::ext::base::ExtCtxt;
use syntax::ptr::P;

pub fn camel_to_snake(mut camel: String) -> String {
    let mut snake = String::new();
    if camel.len() > 0 {
        snake.push(camel.remove(0).to_lowercase().next().unwrap())
    }
    for c in camel.chars() {
        if c.is_uppercase() {
            snake.push('_');
        }
        snake.push(c.to_lowercase().next().unwrap())
    }
    return snake;
}

pub fn snake_to_camel(snake: String) -> String {
    let mut camel = String::new();
    let mut next_is_upper = false;
    for c in snake.chars() {
        // convert next to upper
        if c == '_' {
            next_is_upper = true;
        } else {
            if next_is_upper {
                camel.push(c.to_uppercase().next().unwrap());
                next_is_upper = false;
            } else {
                camel.push(c);
            }
        }
    }
    return snake;
}

pub fn struct_ty(cx: &mut ExtCtxt,
                 span: Span,
                 name: ast::Ident,
                 generics: &ast::Generics)
                 -> P<ast::Ty> {
    let lifetimes = generics.lifetimes.iter().map(|lt| lt.lifetime).collect();
    let ty_params = generics.ty_params
        .iter()
        .map(|param| cx.ty_ident(span, param.ident))
        .collect();
    cx.ty_path(cx.path_all(span, false, vec![name], lifetimes, ty_params, Vec::new()))
}

pub fn extract_generics_from_item(item: &ast::Item) -> Option<ast::Generics> {
    match item.node {
        ItemKind::Struct(_, ref generics) => Some(generics.clone()),
        _ => None,
    }
}
