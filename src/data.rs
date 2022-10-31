
use proc_macro::{Literal, Ident};

#[derive(Debug)]
pub enum Pattern {
    Wild,
/*    Literal(Literal),
    List(Vec<Pattern>, Option<Box<Pattern>>), 
    Tuple(Vec<Pattern>),
    Variable(String),
    At(String, Box<Pattern>),*/
}

// ranges
// .. in tuples, structs, list
// |
// namespaces 
// if
// & pattern
// ! pattern
