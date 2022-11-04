
mod data;
mod parsing;

use proc_macro::{TokenStream, TokenTree};

use crate::data::*;
use crate::parsing::parse_object_pattern;


#[proc_macro]
pub fn object_pattern(input : TokenStream) -> TokenStream {

    let input = input.into_iter().collect::<Vec<_>>();
    let input = input.iter();
    let _x = parse_object_pattern(input).unwrap();

    "".parse().unwrap()
}

/*

Blarg(!, _); "blah" => { }

let mut ret = vec![];
match x {
    Blarg(next_1, _) => {
        match next_1 {
            "blah" => ret.push( block ); // TODO is that possible?
            _ => { } 
        }
         
        match next_2 {
            "blah" => ret.push(block);
            _ => { }
        }
    },
    _ => 
}



*/