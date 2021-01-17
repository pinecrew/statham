extern crate proc_macro;

use std::str::FromStr;
use proc_macro::*;

#[proc_macro]
pub fn json(_tokens: TokenStream) -> TokenStream {
    // TODO!
    // println!("{:?}", tokens);
    TokenStream::from_str("Json::new()").unwrap()
}