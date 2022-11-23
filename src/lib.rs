
mod data;
mod parsing;
mod gen;

use proc_macro::TokenStream;

use crate::data::GenSym;
use crate::parsing::object_pattern::parse_object_pattern;
use crate::gen::object_pattern::object_pattern_matcher;

#[proc_macro]
pub fn object_pattern(input : TokenStream) -> TokenStream {

    let input = input.into_iter().collect::<Vec<_>>();
    let input = input.iter();
    let obj_pat_with_action = parse_object_pattern(input).unwrap();

    let mut g = GenSym::new();

    let o = object_pattern_matcher(&mut g, obj_pat_with_action);

    o.parse().unwrap()
}

