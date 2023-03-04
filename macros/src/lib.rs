extern crate proc_macro;
extern crate syn;
extern crate quote;

use proc_macro::TokenStream;
use syn::{parse_macro_input, parse_quote, DeriveInput, Error, Ident, Path, Result};
use quote::quote;

// pub trait Component: Send + Sync {}

#[proc_macro_derive(Component)]
pub fn derive_component(input: TokenStream) -> TokenStream {
    let mut ast = parse_macro_input!(input as DeriveInput);
    // let bevy_ecs_path: Path = crate::bevy_ecs_path();

    /*
    let attrs = match parse_component_attr(&ast) {
        Ok(attrs) => attrs,
        Err(e) => return e.into_compile_error().into(),
    };

    let storage = storage_path(&bevy_ecs_path, attrs.storage);
    */

    ast.generics
        .make_where_clause()
        .predicates
        .push(parse_quote! { Self: Send + Sync + 'static });

    let struct_name = &ast.ident;
    let (impl_generics, type_generics, where_clause) = &ast.generics.split_for_impl();

    TokenStream::from(quote! {
        impl #impl_generics Component for #struct_name #type_generics #where_clause {
            /*
            fn push_none(&mut self) {
                self.push(None)
            }
            */
        }
    })
}