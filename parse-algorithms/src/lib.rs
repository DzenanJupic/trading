extern crate proc_macro;

use proc_macro::TokenStream;
use proc_macro2::{self, TokenStream as TokenStream2};
use syn::parse;
use quote::quote;


#[proc_marco_attribute]
fn parse_algorithms(_attrs: TokenStream, item: TokenStream) -> TokenStream {

}
