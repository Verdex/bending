
use proc_macro::{TokenStream, TokenTree, Delimiter};

use motif::*;
use denest::*;

use crate::data::*;

pub fn parse_object_pattern<'a>( input : (impl Iterator<Item = &'a TokenTree> + Clone) ) -> Result<ObjPatsAct, MatchError> {
    pred!(not_zero: usize = |x| x != 0);
    seq!(legit_sequence: usize => () = * not_zero, 0, { () });

    let mut input = input.enumerate();
    let pats = obj_pat_with_action(&mut input)?;
    if input.count() != 0 {
        return Err(MatchError::FatalEndOfFile);
    }

    let mut next_counts = pats.obj_pats.iter().map(|pat| pat.to_lax().filter(|x| matches!(x, ObjectPattern::Next)).count()).enumerate();

    legit_sequence(&mut next_counts)?;
    if next_counts.count() != 0 {
        return Err(MatchError::FatalEndOfFile);
    }

    Ok(pats)
}

pred!(colon<'a>: &'a TokenTree = |x| match x { TokenTree::Punct(p) => p.as_char() == ':', _ => false });
pred!(or<'a>: &'a TokenTree = |x| match x { TokenTree::Punct(p) => p.as_char() == '|', _ => false });
pred!(equal<'a>: &'a TokenTree = |x| match x { TokenTree::Punct(p) => p.as_char() == '=', _ => false });
pred!(at<'a>: &'a TokenTree = |x| match x { TokenTree::Punct(p) => p.as_char() == '@', _ => false });
pred!(comma<'a>: &'a TokenTree = |x| match x { TokenTree::Punct(p) => p.as_char() == ',', _ => false });
pred!(semi_colon<'a>: &'a TokenTree = |x| match x { TokenTree::Punct(p) => p.as_char() == ';', _ => false });

group!(arrow<'a>: &'a TokenTree => () = |input| {
    pred!(greater<'a>: &'a TokenTree = |x| match x { TokenTree::Punct(p) => p.as_char() == '>', _ => false });
    seq!(main<'a>: &'a TokenTree => () = equal, ! greater, { () });
    main(input)
});

group!(colon_colon<'a>: &'a TokenTree => &'static str = |input| {
    pred!(colon<'a>: &'a TokenTree = |x| match x { TokenTree::Punct(p) => p.as_char() == ':', _ => false });
    seq!(main<'a>: &'a TokenTree => &'static str = colon, ! colon, { "::" });
    main(input)
});

group!(dot_dot<'a>: &'a TokenTree => ObjectPattern = |input| {
    pred!(dot<'a>:&'a TokenTree = |x| match x { TokenTree::Punct(p) => p.as_char() == '.', _ => false });
    seq!(main<'a>: &'a TokenTree => ObjectPattern = dot, ! dot, { ObjectPattern::Rest });
    main(input)
});

group!(object_pattern<'a>: &'a TokenTree => Vec<ObjectPattern> = |input| {

    group!(structure<'a>: &'a TokenTree => ObjectPattern = |input| {

        seq!( name<'a>: &'a TokenTree => String 
            = ident <= TokenTree::Ident(_) 
            , {
                if let TokenTree::Ident(ident) = ident {
                    ident.to_string()
                }
                else {
                    unreachable!();
                }
            });

        group!(structure_fields<'a>: &'a TokenTree => (Vec<(String, ObjectPattern)>, bool) = |input| {
            seq!(extract<'a>: &'a TokenTree => Option<TokenStream> = group <= TokenTree::Group(_), {
                if let TokenTree::Group(g) = group {
                    if g.delimiter() == Delimiter::Brace {
                        Some(g.stream())
                    }
                    else {
                        None
                    }
                }
                else { 
                    unreachable!();
                }
            });

            seq!(field<'a>: &'a TokenTree => (String, ObjectPattern) 
                = ident <= TokenTree::Ident(_)
                , colon
                , opt <= option
                , { 
                    
                    if let TokenTree::Ident(ident) = ident {
                        let name = ident.to_string();
                        (name, opt)
                    }
                    else {
                        unreachable!();
                    }
                });

            seq!(field_comma<'a>: &'a TokenTree => (String, ObjectPattern) 
                = f <= field
                , comma
                , { f });

            seq!( fields<'a>: &'a TokenTree => (Vec<(String, ObjectPattern)>, bool) 
                = fs <= * field_comma
                , f <= field
                , r <= ? dot_dot
                , { 
                    let mut fs = fs;
                    fs.push(f);
                    (fs, matches!(r, Some(_)))
                });
            
            let group = match extract(input)? {
                Some(g) => g,
                None => { return Err(MatchError::ErrorEndOfFile); },
            };

            let input = group.into_iter().collect::<Vec<TokenTree>>();

            if input.len() == 0 {
                Err(MatchError::FatalEndOfFile)
            }
            else {
                let mut input = input.iter().enumerate();
                let (fs, rest) = fields(&mut input)?;
                Ok((fs, rest))
            }
        });

        seq!( main<'a>: &'a TokenTree => ObjectPattern 
            = n <= name 
            , fs <= structure_fields 
            , {
                let (fields, rest) = fs;
                ObjectPattern::Struct { name: n, fields, rest }
            });
        
        main(input)
    });

    group!(range<'a>: &'a TokenTree => ObjectPattern = |input| {
        seq!(range_inclusive<'a>: &'a TokenTree => ObjectPattern = s <= TokenTree::Literal(_), dot_dot, ! equal, e <= ! TokenTree::Literal(_), {
            if let (TokenTree::Literal(start), TokenTree::Literal(end)) = (s, e) {
                let start = start.to_string();
                let end = end.to_string();
                ObjectPattern::RangeInclusive { start, end }
            }
            else {
                unreachable!();
            }
        });
        range_inclusive(input)
    });

    seq!(at_pat<'a>: &'a TokenTree => ObjectPattern = ident <= TokenTree::Ident(_), at, pattern <= ! option, {
        if let TokenTree::Ident(name) = ident {
            let name = name.to_string();
            let pattern = Box::new(pattern);
            ObjectPattern::At { name, pattern }
        }
        else {
            unreachable!()
        }
    });

    pred!(wild<'a>: &'a TokenTree => ObjectPattern = |x| match x { TokenTree::Ident(n) => n.to_string() == "_", _ => false } => {
        ObjectPattern::Wild
    });

    pred!(bang<'a>: &'a TokenTree => ObjectPattern = |x| match x { TokenTree::Punct(p) => p.as_char() == '!',  _ => false } => {
        ObjectPattern::Next
    });

    seq!(literal<'a>: &'a TokenTree => ObjectPattern = lit <= TokenTree::Literal(_), { 
        if let TokenTree::Literal(lit) = lit {
            ObjectPattern::Literal(lit.to_string())
        }
        else {
            unreachable!()
        }
    });

    seq!(ident_colon_colon<'a>: &'a TokenTree => String = ident <= TokenTree::Ident(_), colon_colon, {
        if let TokenTree::Ident(i) = ident {
            format!( "{}{}", i.to_string(), "::" )
        }
        else {
            unreachable!();
        }
    });

    seq!(cons_tag<'a>: &'a TokenTree => String = prefix <= ? colon_colon
                                               , body <= * ident_colon_colon
                                               , last <= TokenTree::Ident(_)
                                               , {
        if let TokenTree::Ident(i) = last {
            format!( "{}{}{}", prefix.or(Some("")).unwrap()
                             , body.into_iter().collect::<String>()
                             , i.to_string() )
        }
        else {
            unreachable!();
        }
    });

    group!(params<'a>: &'a TokenTree => Vec<ObjectPattern> = |input| {
        alt!(opts<'a>: &'a TokenTree => ObjectPattern = option | dot_dot);
        seq!(pat_comma<'a>: &'a TokenTree => ObjectPattern = pat <= opts, comma, { pat });
        seq!(pat_list<'a>: &'a TokenTree => Vec<ObjectPattern> = first <= * pat_comma, last <= ! opts, {
            let mut first = first;
            first.push(last);
            first
        });

        seq!(extract<'a>: &'a TokenTree => Option<TokenStream> = group <= TokenTree::Group(_), {
            if let TokenTree::Group(g) = group {
                if g.delimiter() == Delimiter::Parenthesis {
                    Some(g.stream())
                }
                else {
                    None
                }
            }
            else { 
                unreachable!();
            }
        });

        let group = match extract(input)? {
            Some(g) => g,
            None => { return Err(MatchError::ErrorEndOfFile); },
        };

        let input = group.into_iter().collect::<Vec<TokenTree>>();

        if input.len() == 0 {
            Ok(vec![])
        }
        else {
            let mut input = input.iter().enumerate();

            let list = pat_list(&mut input)?;

            if input.count() != 0 {
                Err(MatchError::FatalEndOfFile)
            }
            else {
                Ok(list)
            }
        }
    });

    seq!(tuple<'a>: &'a TokenTree => ObjectPattern = ps <= params, { ObjectPattern::Tuple(ps) });

    seq!(cons_with_param<'a>: &'a TokenTree => ObjectPattern = tag <= cons_tag, ps <= params, {
        ObjectPattern::Cons { cons: tag, params: ps }
    });

    seq!(cons_alone<'a>: &'a TokenTree => ObjectPattern = tag <= cons_tag, {
        ObjectPattern::Cons { cons: tag, params: vec![] }
    });

    alt!(cons<'a>: &'a TokenTree => ObjectPattern = cons_with_param | cons_alone);

    group!(option<'a>: &'a TokenTree => ObjectPattern = |input| {
        alt!(item<'a>: &'a TokenTree => ObjectPattern = at_pat 
                                                      | wild
                                                      | range
                                                      | structure
                                                      | literal
                                                      | bang
                                                      | cons
                                                      | tuple
                                                      );

        seq!(item_or<'a>: &'a TokenTree => ObjectPattern = x <= item, or, { x });
        seq!(main<'a>: &'a TokenTree => ObjectPattern = items <= * item_or
                                                      , last <= ! item
                                                      , {

            let mut items = items;
            items.push(last);

            ObjectPattern::Or(items)
        });
        
        main(input)
    });

    seq!(option_semi<'a>: &'a TokenTree => ObjectPattern = o <= option
                                                         , maybe_if <= ? TokenTree::Group(_)
                                                         , semi_colon
                                                         , { 
        match maybe_if {
            Some(TokenTree::Group(g)) => 
                ObjectPattern::If { condition: g.to_string(), pattern: Box::new(o) },
            Some(_) => unreachable!(),
            None => o,
        }
    });
    
    seq!(options<'a>: &'a TokenTree => Vec<ObjectPattern> = os <= * option_semi
                                                          , o <= ! option
                                                          , maybe_if <= ? TokenTree::Group(_)
                                                          , {
        let o = match maybe_if {
            Some(TokenTree::Group(g)) => 
                ObjectPattern::If { condition: g.to_string(), pattern: Box::new(o) },
            Some(_) => unreachable!(),
            None => o,
        };
        let mut os = os;
        os.push(o);
        os
    });

    options(input)
});

seq!(obj_pat_with_action<'a>: &'a TokenTree => ObjPatsAct 
    = obj_pats <= object_pattern, arrow, g <= TokenTree::Group(_), { 
        if let TokenTree::Group(g) = g {
            ObjPatsAct { obj_pats, action : g.to_string() }
        }
        else {
            unreachable!();
        }
    });