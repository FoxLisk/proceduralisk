extern crate proc_macro;
use proc_macro::TokenStream;

use syn::{parse_macro_input, Result, Token};
use syn::parse::{Parse, ParseStream};
use syn::LitInt;



struct RepeatInput {
    times: LitInt,
    tt: proc_macro2::TokenStream
}

impl Parse for RepeatInput {
    fn parse(input: ParseStream) -> Result<Self> {
        let times =         syn::LitInt::parse(input)?;
        let _comma = <Token![,]>::parse(input)?;
        Ok(RepeatInput{
          times,
            tt: input.parse()?
        })
    }
}

#[proc_macro]
pub fn repeat(input: TokenStream) -> TokenStream {
    //
    let input = parse_macro_input!(input as RepeatInput);
    let mut t = proc_macro2::TokenStream::new();
    let times = input.times.base10_parse().unwrap();
    for _ in 0..times {
        t.extend(input.tt.clone());
    }
    t.into()

}
