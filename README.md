
# Bending

## Object Patterns 

### Introduction

Object patterns allow chaining multiple patterns together in a linear fashion that would otherwise require nested match statements.

    object_pattern!( <Pattern_1>; <Pattern_2>; <Pattern_3> => { <Output> });

Which would otherwise require the nested match statement:

    let mut ret = vec![];
    match ? {
        <Pattern_1> => {
            match ? {
                <Pattern_2> => {
                    match ? {
                        <Pattern_3> => {
                            ret.push( { <Output> });
                        },
                        _ => { },
                    }
                },
                _ => { },
            }
        },
        _ => { },
    }

### Usage 

The `object_pattern` macro produces anonymous function that takes some type as input and returns a `Vec` of some output.  In most cases you'll either want to or need to assign type information to the output.

    let matcher : for<'a> fn(&'a List<u8>) -> Vec<&'a u8>
        = object_pattern!( Cons(x, _) => { x });

### Next Patterns

Object patterns introduce a 'Next' pattern (`!`) that allows you to chain the items from one pattern in the sequence to the next.

    object_pattern!( Cons(_, !); Cons(!, _); x => { x } )

    // Cons(1, Cons(2, Nil)) => [2] 

In the even that more than one 'Next' pattern occurs in a given pattern, then the object pattern will attempt to match all possibilities.

    object_pattern!( TreeNode(!, !); Leaf(x) => { x } )

    // TreeNode( Leaf(1), Leaf(2) ) => [1, 2]

Because the whole point is chaining the output from one pattern to the next pattern, all leading patterns in an object pattern must have at least one 'Next' pattern within it.  And the final pattern may not have any 'Next' patterns.


### Conditionals

Object patterns may include a trailing conditional expression that can be used to guard the next step.

    object_pattern!( Cons(x @ !, _) ? { *x % 2 == 0 } => ... )

### Executables

Object patterns may include a trailing executable statement which will be resolved after the current pattern is matched successfully.

    object_pattern!( Cons( x @ !, _ ) & { let y = *x + 1; } ... )

Conditionals and Executables can be mixed, but the conditional must come first.

    object_pattern!( Cons( x @ !, _ ) ? { *x % 2 == 0 } & { let y = *x + 1; } ... )

