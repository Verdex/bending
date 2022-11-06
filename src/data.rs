
use denest::*;

#[derive(Debug)]
pub struct ObjPatsAct {
    pub obj_pats : Vec<ObjectPattern>,
    pub action : String,
}

#[derive(Debug)]
pub enum ObjectPattern {
    Wild,
    Next,
    Literal(String),
    Ident(String),
    /*List(Vec<Pattern>, Option<Box<Pattern>>), 
    Tuple(Vec<Pattern>),
    Variable(String),
    At(String, Box<Pattern>),*/
}

impl<'a> Linearizable<'a> for ObjectPattern {
    fn l_next(&'a self) -> Vec<&'a Self> {
        match self {
            ObjectPattern::Wild => vec![],
            ObjectPattern::Next => vec![],
            ObjectPattern::Literal(_) => vec![],
            ObjectPattern::Ident(_) => vec![],
        }
    }
}

/*

it sounds like negative literals are punct + literal and then even sometimes group ( punct + literal )

mut, ref, ref + mut are also possibilities in patterns, but I think we're always going to be operating 
on a reference and then returning a reference.  mutability shouldn't enter into it because we're 
going to be potentially returning the same inner parts of the data many times
    
    ref is about getting a ref to something that isn't, but I don't think we ever want ownership, so we'll always have ref already
    mut is about changing it, but we want to give all possible instances of the pattern, which can include the same data more than once
        so mutating it isn't going to be an option

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
