
use denest::*; 

use crate::data::*;

fn obj_pat_match(pat : String, execute : &str, next : String, prev_names : &Vec<String>) -> String {
    prev_names.iter().map(|prev_name| 
        format!("
        match {name}.borrow() {{
            {pat} => {{ 
                {execute}
                {next} 
            }},
            _ => {{}},
        }}
        "
        , name = prev_name
        , pat = pat
        , execute = execute
        , next = next
        )).collect::<String>()
}

fn obj_pat_to_string(input : &ObjectPattern, next_names : &mut Vec<String>) -> String {
    use ObjectPattern::*;
    match input {
        Wild => "_".into(),
        Rest => "..".into(),
        Next => next_names.pop().expect("ran out of next_names while building object pattern").into(),
        Literal(l) => l.clone(),
        Cons { cons, params } if params.len() == 0 => cons.clone(),
        Cons { cons, params } => format!("{}({})", cons.clone(), 
            params.iter().map(|x| obj_pat_to_string(x, next_names)).collect::<Vec<_>>().join(", ")),
        Tuple ( params ) => format!("({})",  
            params.iter().map(|x| obj_pat_to_string(x, next_names)).collect::<Vec<_>>().join(", ")),
        At { name, pattern } => format!("{} @ {}", name, obj_pat_to_string(pattern, next_names)),
        RangeInclusive { start, end } => format!("{}..={}", start, end),
        If { pattern, condition } => format!("{} if {}", obj_pat_to_string(pattern, next_names), condition),
        Or(pats) => 
            pats.iter().map(|x| obj_pat_to_string(x, next_names)).collect::<Vec<_>>().join(" | "),
        Struct { name, fields, rest: true } => {
            let fields = fields.iter().map(|(name, pat)| format!("{} : {}", name, obj_pat_to_string(pat, next_names)))
                               .collect::<Vec<_>>().join(", ");
            if fields.len() != 0 {
                format!("{} {{ {}, .. }}", name, fields)
            }
            else {
                format!("{} {{ .. }}", name)
            }
        },
        Struct { name, fields, rest: false } => {
            let fields = fields.iter().map(|(name, pat)| format!("{} : {}", name, obj_pat_to_string(pat, next_names)))
                               .collect::<Vec<_>>().join(", ");
            format!("{} {{ {} }}", name, fields)
        },
        List ( items ) => format!("[{}]",  
            items.iter().map(|x| obj_pat_to_string(x, next_names)).collect::<Vec<_>>().join(", ")),
        // Note:  Execute contents will be thrown before matches and do not resolve to any actual pattern.
        Execute { .. } => unreachable!(),
    }
}

pub fn object_pattern_matcher(g : &mut GenSym, input : ObjPatsAct) -> String {
    let ObjPatsAct { obj_pats, action } = input;
    let mut obj_pats = obj_pats.into_iter().map(|x| Some(x)).collect::<Vec<_>>();
    obj_pats.reverse();
    obj_pats.push(None);
    let (mut names, mut next) : (Vec<String>, String) = (vec![], format!( "{{ ret.push( {} ); }}", action.to_string() ));
    for (cur_pat, prev_pat) in obj_pats.iter().zip(obj_pats.iter().skip(1)) {

        let mut cur_names = names;
        let prev_names = 
            match prev_pat {
                Some(p) => p.to_lax().filter(|x| matches!(x, ObjectPattern::Next)).map(|_| g.gen()).collect::<Vec<String>>(),
                None => vec!["gen_sym_input".into()],
            };

        if let Some(ObjectPattern::Execute { action, pattern }) = cur_pat {
            let cur_pat_as_string = obj_pat_to_string(pattern, &mut cur_names);
            next = obj_pat_match(cur_pat_as_string, action, next, &prev_names);
            names = prev_names;
        }
        else {
            let cur_pat_as_string = obj_pat_to_string(cur_pat.as_ref().unwrap(), &mut cur_names);
            next = obj_pat_match(cur_pat_as_string, "", next, &prev_names);
            names = prev_names;
        }
    }

    format!("
    |gen_sym_input| {{
        use std::borrow::Borrow;
        let mut ret = vec![];
        {}
        ret
    }}
    ", next ) 
}
