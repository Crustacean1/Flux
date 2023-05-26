use std::mem::size_of;

use component_allocator::{
    allocator_implementation, allocator_initializer, allocator_struct,
    component_type_implementations, get_component_types,
};
use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DataStruct, Lit};

mod component_allocator;

#[proc_macro_derive(Vertex)]
pub fn vertex_derive(stream: TokenStream) -> TokenStream {
    let struct_ast: syn::DeriveInput = syn::parse(stream).unwrap();

    let name = struct_ast.ident;

    let fields = if let Data::Struct(strct) = struct_ast.data {
        get_field_data(strct)
    } else {
        panic!("Entity deriving 'Vertex' must be a struct: '{}'", name);
    };

    let stride: u32 = fields
        .iter()
        .map(|field_size| field_size * size_of::<f32>() as u32)
        .sum();

    let offsets: Vec<_> = fields
        .iter()
        .scan((0, 0), |(total, prev), &len| {
            *total += *prev;
            *prev = len * size_of::<f32>() as u32;
            Some((len, *total))
        })
        .collect();

    let declarations: Vec<_> = offsets
        .iter()
        .enumerate()
        .map(|(i, (len, offset))| {
            quote! {
                glad_gl::gl::VertexAttribPointer(#i as u32,
                    #len as i32,
                    glad_gl::gl::FLOAT,
                    glad_gl::gl::FALSE,
                    #stride as i32,
                    #offset as *const std::ffi::c_void);
                glad_gl::gl::EnableVertexAttribArray(#i as u32);
            }
        })
        .collect();

    quote! {
        impl Vertex for #name{
            type VertexType = #name;
            fn declare_layout(){
                unsafe{
                    #(#declarations)*
                }
            }

            fn size(len: usize) -> usize {
                len * #stride as usize
            }
        }
    }
    .into()
}

#[proc_macro]
pub fn declare_components(component_stream: TokenStream) -> TokenStream {
    let component_types = get_component_types(component_stream);

    let allocator_fields = allocator_struct(&component_types);
    let allocator_initializer = allocator_initializer(&component_types);
    let allocator_implementations = allocator_implementation(&component_types);

    let component_type_implementations = component_type_implementations(&component_types);

    quote! {
        #(#component_type_implementations)*

        pub struct ConcreteComponentAllocator(#(#allocator_fields),*);

        impl ConcreteComponentAllocator {
            pub fn new() -> Self {
                ConcreteComponentAllocator (#(#allocator_initializer),*)
            }
        }

         #(#allocator_implementations)*
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
