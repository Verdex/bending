
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
    Or(Vec<ObjectPattern>),
    Struct { name: String, fields : Vec<(String, ObjectPattern)>, rest : bool },
    List(Vec<ObjectPattern>),
    Execute { pattern : Box<ObjectPattern>, action : String },
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
            Or(pats) => pats.iter().collect::<Vec<_>>(),
            Struct { fields, .. } => fields.iter().map(|x| &x.1).collect::<Vec<_>>(),
            List(items) => items.iter().collect::<Vec<_>>(),
            Execute { pattern, .. } => vec![pattern],
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
