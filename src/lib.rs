
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

struct GenSym {
    index : u64,
}

impl GenSym {
    fn new() -> Self {
        GenSym { index: 0 }
    }

    fn gen(&mut self) -> String {
        let ret = format!("gen_sym_{}", self.index);
        self.index += 1;
        ret
    }
}

fn gen_object_pattern_matcher<'a>(input : PatternAction<ObjectPattern<'a>>) -> String {
    todo!()
}
//TODO: gensym

/*

Blarg(!, !); "blah" => { }

let mut ret = vec![];
match x {
    Blarg(next_1, next_2) => {
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


Blarg(!, !) => Other(!) => Blarg(!, !) => 5 => { }

match x {
    Blarg(next_1, next_2) => {
        match next_1 {
            Other(next_3) => {
                match next_3 {
                    Blarg(next_4, next_5) => {
                        match next_4 {
                            5 => ret.push(block),
                            _ => { }
                        }
                        match next_5 {
                            5 => ret.push(block),
                            _ => { }
                        }
                    }
                    _ => { }
                }
                _ => { }
            }
            _ => { }
        }

        match next_2 {
            Other(next_6) => {
                match next_6 {
                    Blarg(next_7, next_8) => {
                        match next_7 {
                            5 => ret.push(block),
                            _ => { }
                        }
                        match next_8 {
                            5 => ret.push(block),
                            _ => { }
                        }
                    }
                    _ => { }
                }
                _ => { }
            }
            _ => { }
        }
    }
    _ => { }
}


*/