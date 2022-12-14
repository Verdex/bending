
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

pred!(and<'a>: &'a TokenTree = |x| match x { TokenTree::Punct(p) => p.as_char() == '&', _ => false });
pred!(colon<'a>: &'a TokenTree = |x| match x { TokenTree::Punct(p) => p.as_char() == ':', _ => false });
pred!(or<'a>: &'a TokenTree = |x| match x { TokenTree::Punct(p) => p.as_char() == '|', _ => false });
pred!(equal<'a>: &'a TokenTree = |x| match x { TokenTree::Punct(p) => p.as_char() == '=', _ => false });
pred!(at<'a>: &'a TokenTree = |x| match x { TokenTree::Punct(p) => p.as_char() == '@', _ => false });
pred!(comma<'a>: &'a TokenTree = |x| match x { TokenTree::Punct(p) => p.as_char() == ',', _ => false });
pred!(semi_colon<'a>: &'a TokenTree = |x| match x { TokenTree::Punct(p) => p.as_char() == ';', _ => false });
pred!(minus<'a>: &'a TokenTree = |x| match x { TokenTree::Punct(p) => p.as_char() == '-', _ => false });
pred!(question<'a>: &'a TokenTree = |x| match x { TokenTree::Punct(p) => p.as_char() == '?', _ => false });

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

    group!(list<'a>: &'a TokenTree => ObjectPattern = |input| {
        seq!(extract<'a>: &'a TokenTree => Option<TokenStream> = group <= TokenTree::Group(_), {
            if let TokenTree::Group(g) = group {
                if g.delimiter() == Delimiter::Bracket {
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

        seq!( at_dot_dot<'a>: &'a TokenTree => ObjectPattern 
            = ident <= TokenTree::Ident(_), at, dot_dot, {

            if let TokenTree::Ident(name) = ident {
                let name = name.to_string();
                ObjectPattern::At { name, pattern: Box::new(ObjectPattern::Rest) }
            }
            else {
                unreachable!()
            }
        });


        alt!(item<'a>: &'a TokenTree => ObjectPattern = dot_dot | at_dot_dot | option );
        seq!(item_comma<'a>: &'a TokenTree => ObjectPattern = i <= item, comma, { i });
        seq!(items<'a>: &'a TokenTree => Vec<ObjectPattern> = xs <= * item_comma, x <= ! item, {
            let mut xs = xs;
            xs.push(x);
            xs
        });

        let group = match extract(input)? {
            Some(g) => g,
            None => { return Err(MatchError::ErrorEndOfFile); },
        };

        let input = group.into_iter().collect::<Vec<TokenTree>>();

        if input.len() == 0 {
            Ok(ObjectPattern::List(vec![]))
        }
        else {
            let mut input = input.iter().enumerate();
            let items = items(&mut input)?;

            if input.count() != 0 {
                Err(MatchError::FatalEndOfFile)
            }
            else {
                Ok(ObjectPattern::List(items))
            }
        }
    });

    group!(literal<'a>: &'a TokenTree => ObjectPattern = |input| {
        seq!( literal_alone<'a>: &'a TokenTree => ObjectPattern
            = lit <= TokenTree::Literal(_)
            , {
                if let TokenTree::Literal(lit) = lit {
                    ObjectPattern::Literal(lit.to_string())
                }
                else {
                    unreachable!();
                }
            });
        seq!( minus_literal<'a>: &'a TokenTree => ObjectPattern
            = minus
            , lit <= ! TokenTree::Literal(_)
            , {
                if let TokenTree::Literal(lit) = lit {
                    ObjectPattern::Literal(format!("-{}", lit.to_string()))
                }
                else {
                    unreachable!();
                }
            });

        alt!(main<'a>: &'a TokenTree => ObjectPattern = literal_alone | minus_literal);

        main(input)
    });

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

            seq!(field_alone<'a>: &'a TokenTree => (String, ObjectPattern) 
                = ident <= TokenTree::Ident(_)
                , { 
                    
                    if let TokenTree::Ident(ident) = ident {
                        let name = ident.to_string();
                        (name.clone(), ObjectPattern::Literal(name))
                    }
                    else {
                        unreachable!();
                    }
                });

            seq!(field_with_pattern<'a>: &'a TokenTree => (String, ObjectPattern) 
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

            alt!(field<'a>: &'a TokenTree => (String, ObjectPattern) = field_with_pattern | field_alone);

            seq!(field_comma<'a>: &'a TokenTree => (String, ObjectPattern) 
                = f <= field
                , comma
                , { f });

            seq!( fields_with_rest<'a>: &'a TokenTree => (Vec<(String, ObjectPattern)>, bool) 
                = fs <= * field_comma
                , dot_dot
                , { (fs, true) });

            seq!( fields_alone<'a>: &'a TokenTree => (Vec<(String, ObjectPattern)>, bool) 
                = fs <= * field_comma
                , f <= field
                , { 
                    let mut fs = fs;
                    fs.push(f);
                    (fs, false)
                });
            
            alt!( fields<'a>: &'a TokenTree => (Vec<(String, ObjectPattern)>, bool) = fields_with_rest | fields_alone );
            
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

                if input.count() != 0 {
                    Err(MatchError::FatalEndOfFile)
                }
                else {
                    Ok((fs, rest))
                }
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
        seq!(range_inclusive<'a>: &'a TokenTree => ObjectPattern = s <= literal, dot_dot, ! equal, e <= ! literal, {
            if let (ObjectPattern::Literal(start), ObjectPattern::Literal(end)) = (s, e) {
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
                                                      | list
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

    seq!(condition<'a>: &'a TokenTree => String = question, tt <= ! TokenTree::Group(_), { tt.to_string() });

    seq!(execute<'a>: &'a TokenTree => String = and, tt <= ! TokenTree::Group(_), {
        if let TokenTree::Group(g) = tt {
            g.stream().to_string()
        }
        else {
            unreachable!();
        }
    });

    seq!(option_semi<'a>: &'a TokenTree => ObjectPattern = o <= option
                                                         , maybe_if <= ? condition 
                                                         , maybe_execute <= ? execute
                                                         , semi_colon
                                                         , { 
        match (maybe_if, maybe_execute) {
            (Some(condition), Some(action)) => 
                ObjectPattern::Execute { 
                    action, 
                    pattern: Box::new(ObjectPattern::If { condition, pattern: Box::new(o) }),
                },
            (Some(condition), None) => 
                ObjectPattern::If { condition, pattern: Box::new(o) },
            (None, Some(action)) => 
                ObjectPattern::Execute { 
                    action,
                    pattern: Box::new(o),
                },
            (None, None) => o,
        }
    });
    
    seq!(options<'a>: &'a TokenTree => Vec<ObjectPattern> = os <= * option_semi
                                                          , o <= ! option
                                                          , maybe_if <= ? condition
                                                          , maybe_execute <= ? execute
                                                          , {

        let o = match (maybe_if, maybe_execute) {
            (Some(condition), Some(action)) => 
                ObjectPattern::Execute { 
                    action,
                    pattern: Box::new(ObjectPattern::If { condition, pattern: Box::new(o) }),
                },
            (Some(condition), None) => 
                ObjectPattern::If { condition, pattern: Box::new(o) },
            (None, Some(action)) => 
                ObjectPattern::Execute { 
                    action,
                    pattern: Box::new(o),
                },
            (None, None) => o,
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