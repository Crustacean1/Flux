use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DataStruct, Lit};

#[proc_macro_derive(Vertex)]
pub fn vertex_derive(stream: TokenStream) -> TokenStream {
    let struct_ast: syn::DeriveInput = syn::parse(stream).unwrap();

    let name = struct_ast.ident;

    let fields = if let Data::Struct(strct) = struct_ast.data {
        get_field_data(strct)
    } else {
        panic!("Entity deriving 'Vertex' must be a struct: '{}'", name);
    };

    let attribute_count = fields.len();
    let stride: u32 = fields.iter().map(|field_size| field_size * 4).sum();
    let offsets: Vec<_> = fields
        .iter()
        .scan(0, |&mut acc, &size| Some(acc + size))
        .collect();

    println!("Defining {} attributes for: {}", attribute_count, name);

    let declarations: Vec<_> = offsets
        .iter()
        .enumerate()
        .map(|(i, offset)| {
            quote! {
                //glad_gl::gl::VertexAttribPointer(#i as u32, #attribute_count, glad_gl::gl::FLOAT, glad_gl::gl::FALSE, #stride, #offset);
                //glad_gl::gl::EnableVertexAttribArray(#i as u32);
                println!("{}/{} : stride: {} offset: {}", #i, #attribute_count, #stride, #offset);
            }
        })
        .collect();

    quote! {
        impl Vertex for #name{
            type VertexType = #name;
            fn declare_layout(){
                #(#declarations)*
            }
        }
    }
    .into()
}

fn get_field_data(strct: DataStruct) -> Vec<u32> {
    match strct.fields {
        syn::Fields::Named(named_fields) => named_fields
            .named
            .iter()
            .map(|field| match &field.ty {
                syn::Type::Array(attribute) => match attribute.len.clone() {
                    syn::Expr::Lit(length) => match length.lit {
                        Lit::Int(length) => length.base10_parse().unwrap(),
                        _ => panic!("Invalid length value"),
                    },
                    _ => panic!("Invalid len"),
                },
                _ => panic!("Invalid type"),
            })
            .collect(),
        syn::Fields::Unnamed(_) => todo!(),
        syn::Fields::Unit => todo!(),
    }
}
