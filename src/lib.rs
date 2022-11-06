
mod data;
mod parsing;

use proc_macro::TokenStream;

use crate::data::*;
use crate::parsing::parse_object_pattern;

#[proc_macro]
pub fn object_pattern(input : TokenStream) -> TokenStream {

    let input = input.into_iter().collect::<Vec<_>>();
    let input = input.iter();
    let obj_pat_with_action = parse_object_pattern(input).unwrap();

    let mut g = GenSym::new();

    let o = gen_object_pattern_matcher(&mut g, obj_pat_with_action);
    println!("output = {}", o);

    o.parse().unwrap()
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

use denest::*; // TODO

fn obj_pat_match(pat : String, next : String, prev_names : &Vec<String>) -> String {
    prev_names.iter().map(|prev_name| 
        format!("
        match {name} {{
            {pat} => {next},
            _ => {{}},
        }}
        "
        , name = prev_name
        , pat = pat
        , next = next
        )).collect::<String>()
}

fn obj_pat_to_string(input : &ObjectPattern, mut next_names : Vec<String>) -> String {
    match input {
        ObjectPattern::Wild => "_".into(),
        ObjectPattern::Next => next_names.pop().expect("ran out of next_names while building object pattern").into(),
        ObjectPattern::Literal(l) => l.clone(),
    }
}

fn gen_object_pattern_matcher(g : &mut GenSym, input : ObjPatsAct) -> String {
    let ObjPatsAct { obj_pats, action } = input;
    let mut obj_pats = obj_pats.into_iter().map(|x| Some(x)).collect::<Vec<_>>();
    obj_pats.reverse();
    obj_pats.push(None);
    let (mut names, mut next) : (Vec<String>, String) = (vec![], format!( "{{ ret.push( {} ); }}", action.to_string() ));
    for (cur_pat, prev_pat) in obj_pats.iter().zip(obj_pats.iter().skip(1)) {

        let cur_names = names;
        let prev_names = 
            match prev_pat {
                Some(p) => p.to_lax().filter(|x| matches!(x, ObjectPattern::Next)).map(|_| g.gen()).collect::<Vec<String>>(),
                None => vec!["input".into()],
            };

        let cur_pat_as_string = obj_pat_to_string(cur_pat.as_ref().unwrap(), cur_names);
        next = obj_pat_match(cur_pat_as_string, next, &prev_names);
        println!("next = {}", next);
        names = prev_names;
    }

    format!("
    |input| {{
        let mut ret = vec![];
        {}
        ret
    }}
    ", next ) 
}

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