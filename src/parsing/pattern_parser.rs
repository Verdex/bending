
use proc_macro::TokenTree;
use crate::data::Pattern;
use super::data::ParseResult;

macro_rules! input {
    () => { &mut (impl Iterator<Item = TokenTree> + Clone) };
}

pub fn parse(input : input!()) -> ParseResult<Pattern> {
    let mut rp = input.clone();

    match input.next() {
        Some(TokenTree::Ident(x)) if x.to_string() == "_" => ParseResult::Success(Pattern::Wild),
        None => ParseResult::Error,
        _ => todo!(),
    }
}
