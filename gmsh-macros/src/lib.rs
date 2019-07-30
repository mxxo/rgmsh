/// Macro for duplicated functions in Gmsh interface

extern crate quote;
extern crate syn;
extern crate proc_macro;

use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn geometry_kernel(attr: TokenStream, item: TokenStream) -> TokenStream {
    println!("{}", attr);
    println!("{}", item);
    item
}


