extern crate proc_macro;
use proc_macro::TokenStream;

trait Component: Send + Sync {}

#[proc_macro_derive(Component)]
pub fn derive_component(_item: TokenStream) -> TokenStream {
    "fn answer() -> u32 { 42 }".parse().unwrap()
}