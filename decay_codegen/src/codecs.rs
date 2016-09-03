// Copyright 2016 Jeremy Letang.
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use aster::ident::ToIdent;
use syntax::ext::base::ExtCtxt;
use syntax::codemap::Span;
use syntax::ast::{MetaItem, Path, MetaItemKind,
                  NestedMetaItemKind, PathParameters,
                  PathSegment};

pub fn make_segments(segs: Vec<String>) -> Vec<PathSegment> {
    segs.into_iter().map(|s|
                         PathSegment{
                             identifier: s.to_ident(),
                             parameters: PathParameters::none(),
                         }
    ).collect()
}

fn build_path_to_codec(span: &Span, s: String) -> Path {
    Path {
        span: span.clone(),
        global: true,
        segments: make_segments(vec![s, "Codec".to_string()]),
    }
}

pub fn extract_codecs_from_meta_item(ecx: &mut ExtCtxt, meta_item: &MetaItem) -> Vec<Path>{
    match &meta_item.node {
        &MetaItemKind::List(_, ref l) => {
            l.iter().map(|e| {
                match &e.node {
                    &NestedMetaItemKind::MetaItem(ref s) => {
                        match s.node {
                            MetaItemKind::Word(ref w) => build_path_to_codec(&meta_item.span, w.to_string()),
                            _ => ecx.span_fatal(meta_item.span, "attributes value must be words")
                        }
                    },
                    _ => ecx.span_fatal(meta_item.span, "attributes value must be words")
                }
            }).collect()
        },
        _ => ecx.span_fatal(meta_item.span, "attribute must be a list")
    }
}
