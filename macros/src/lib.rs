use proc_macro::{TokenStream, TokenTree};
use quote::quote;
use syn::{self, Data::Struct};

#[proc_macro_attribute]
pub fn annotate(_: TokenStream, item: TokenStream) -> TokenStream {
    let item_ast: syn::DeriveInput = syn::parse(item).unwrap();
    let item_name = item_ast.ident.clone();

    let items = match item_ast.data.clone() {
        Struct(data) => data.fields,
        _ => panic!("Cannot annotate type: '{}', must be a struct", item_name),
    };

    let show_vars = items
        .iter()
        .filter(|field| field.ident.is_some())
        .map(|field| {
            let field_name = &field.ident;
            quote! {
                println!("Field: {} = {}", stringify!(#field_name),self.#field_name);
            }
        });

    let gen = quote! {
        #item_ast
        impl #item_name {
            pub fn serialize(&self){
                #(#show_vars )*
            }
        }
    };

    gen.into()
}
