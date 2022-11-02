
use proc_macro::{Literal, Ident};

#[derive(Debug)]
pub enum Pattern<'a> {
    Wild,
    Literal(&'a Literal),
    /*List(Vec<Pattern>, Option<Box<Pattern>>), 
    Tuple(Vec<Pattern>),
    Variable(String),
    At(String, Box<Pattern>),*/
}

/*

match _ {
    1 => 
    1 | 2 => 
    1..2 =>
    1..=2 =>
    a =>
    a @ _ =>
    Blah =>
    Blah(P) =>
    Blah(P, ..) => ??
    Blah { a : P } => 
    Blah { a : P, .. } => 
    [] =>
    [P, P] => 
    [P, ..] =>
    [P, a @ ..] =>
    A::B => (and etc)
    () => ??
    (a, b) =>
    (a, .., b) =>
    P if pred =>
    & name =>
    ! =>
    ! & name =>
}


*/

// ranges
// .. in tuples, structs, list
// |
// namespaces 
// if
// & pattern
// ! pattern
