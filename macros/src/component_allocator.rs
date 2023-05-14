use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::Ident;

pub fn get_component_types(stream: TokenStream) -> Vec<Ident> {
    let input: syn::TypeTuple =
        syn::parse(stream).expect("Component declaration must be a type tuple");
    input
        .elems
        .iter()
        .filter_map(|component_type| match component_type {
            syn::Type::Path(strct) => match strct.path.segments.last() {
                Some(name) => Some(name.ident.clone()),
                None => None,
            },
            _ => {
                panic!("All components in declaration must be structs");
            }
        })
        .collect()
}

pub fn component_type_implementations(component_types: &[Ident]) -> Vec<impl ToTokens> {
    component_types
        .iter()
        .enumerate()
        .map(|(i, component_type)| {
            quote! {
                impl ComponentType for #component_type{
                    fn component_type_id() -> usize {
                        #i
                    }
                }
            }
        })
        .collect()
}

pub fn allocator_initializer(component_types: &[Ident]) -> Vec<impl ToTokens> {
    component_types
        .iter()
        .map(|component_type| {
            quote! {
                ComponentAllocator::<#component_type>::new()
            }
        })
        .collect()
}

pub fn allocator_struct(component_types: &[Ident]) -> Vec<impl ToTokens> {
    component_types
        .iter()
        .map(|component_type| {
            quote! {
                ComponentAllocator<#component_type>
            }
        })
        .collect()
}

pub fn allocator_implementation(component_types: &[Ident]) -> Vec<impl ToTokens> {
    component_types
        .iter()
        .enumerate()
        .map(|(i, component_type)| {
            let index = syn::Index::from(i);
            quote! {
                impl AllocatorManager<#component_type> for ConcreteComponentAllocator{
                    fn allocator(&mut self) -> &mut ComponentAllocator<#component_type> {
                        &mut self.#index
                    }
                }
            }
        })
        .collect()
}
