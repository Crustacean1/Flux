use proc_macro::TokenStream;
use proc_macro::{self};
use quote::quote;
use syn::{self, Data, DeriveInput};

#[proc_macro_derive(BufferLayout)]
pub fn layout_derive(input: TokenStream) -> TokenStream {
    let ast: DeriveInput = syn::parse(input).expect("Couldn't parse input to layout_derive");
    let struct_name = ast.ident;
    let _struct_generic = ast
        .generics
        .params
        .first()
        .map(|generic| match generic {
            syn::GenericParam::Type(gen_type) => &gen_type.ident,
            _ => {
                panic!("Invalid generic type")
            }
        })
        .expect("Invalid generics");
    let Data::Struct(struct_info) = ast.data else {panic!("Couldn't implement VertexLayout for type other than struct")};

    let count = struct_info.fields.iter().map(|field| {
        let field_type = &field.ty;
        quote! {crate::graphics::vertices::layouts::BufferLayout::<#field_type>::count()}
    });

    let sum_count = count.clone();
    let sum = quote! {#(#sum_count+)*};

    quote! {
        impl BufferLayout<f32> for #struct_name{
            const COUNT: usize = #sum;

            fn layout() -> Vec<usize>{
                vec![#(#count,)*]
            }
        }
    }
    .into()
}
