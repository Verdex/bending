
mod data;
mod parsing;

use proc_macro::{TokenStream, TokenTree};


#[proc_macro]
pub fn blarg(x : TokenStream) -> TokenStream {
    /*let mut x = x.into_iter();
    let z = x.next();

    match z.unwrap() {
        TokenTree::Punct(p) => println!("!{}", p.as_char()),
        _ => {},
    }

    let z = x.next();

    match z.unwrap() {
        TokenTree::Punct(p) => println!("!{}", p.as_char()),
        _ => {},
    }*/

    for w in x {

        println!("~~~ {:?}", w.span());
    }

    "".parse().unwrap()
}

//#[proc_macro]
//fn parse_pat(x : TokenStream) -> Result<
