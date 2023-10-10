use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn con(attr: TokenStream, item: TokenStream) -> TokenStream {
    item
}