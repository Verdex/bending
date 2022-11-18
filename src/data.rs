
use denest::*;

#[derive(Debug)]
pub struct ObjPatsAct {
    pub obj_pats : Vec<ObjectPattern>,
    pub action : String,
}

#[derive(Debug)]
pub enum ObjectPattern {
    Wild,
    Rest,
    Next,
    Literal(String),
    Cons { cons : String, params : Vec<ObjectPattern> },
    Tuple(Vec<ObjectPattern>),
    At { name : String, pattern : Box<ObjectPattern> },
    RangeInclusive { start : String, end : String },
    If { pattern : Box<ObjectPattern>, condition : String },
    /*List(Vec<Pattern>, Option<Box<Pattern>>), 
    structure*/
}

impl<'a> Linearizable<'a> for ObjectPattern {
    fn l_next(&'a self) -> Vec<&'a Self> {
        use ObjectPattern::*;
        match self {
            Wild => vec![],
            Rest => vec![],
            Next => vec![],
            Literal(_) => vec![],
            Cons { params, .. } => params.iter().collect::<Vec<_>>(),
            Tuple ( params ) => params.iter().collect::<Vec<_>>(),
            At { pattern, .. } => vec![pattern],
            RangeInclusive { .. } => vec![],
            If { pattern, .. } => vec![pattern],
        }
    }
}

pub struct GenSym {
    index : u64,
}

impl GenSym {
    pub fn new() -> Self {
        GenSym { index: 0 }
    }

    pub fn gen(&mut self) -> String {
        let ret = format!("gen_sym_{}", self.index);
        self.index += 1;
        ret
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
    1 | 2 => 
    Blah { a : P } => 
    Blah { a : P, .. } => 
    [] =>
    [P, P] => 
    [P, ..] =>
    [P, a @ ..] =>
    & name =>
    ! & name =>
}


*/

// .. in tuples, structs, list
// |
// & pattern
