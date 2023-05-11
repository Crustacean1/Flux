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
            let component_name: Ident = syn::parse_str(&component_type.to_string().to_lowercase())
                .expect("Component Type should be a valid identifier");
            quote! {
                #component_name: ComponentAllocator::<#component_type>::new()
            }
        })
        .collect()
}

pub fn allocator_struct(component_types: &[Ident]) -> Vec<impl ToTokens> {
    component_types
        .iter()
        .map(|component_type| {
            let component_name: Ident = syn::parse_str(&component_type.to_string().to_lowercase())
                .expect("Component Type should be a valid identifier");

            quote! {
                #component_name: ComponentAllocator<#component_type>
            }
        })
        .collect()
}

pub fn allocator_implementation(component_types: &[Ident]) -> Vec<impl ToTokens> {
    component_types
        .iter()
        .map(|component_type| {
            let component_name: Ident = syn::parse_str(&component_type.to_string().to_lowercase())
                .expect("Component Type should be a valid identifier");

            quote! {
                impl GenericComponentAllocator<#component_type> for ConcreteComponentAllocator{
                    fn add_component(&mut self, entity_id: usize, value: #component_type) -> usize {
                        self.#component_name.add_component(entity_id, value)
                    }

                    fn remove_component(&mut self, entity_id: usize) {
                        self.#component_name.remove_component(entity_id);
                    }

                    fn get_component_mut(&mut self, entity_id: usize) -> Option<&mut Component<#component_type>> {
                        self.#component_name.get_component_mut(entity_id)
                    }
                }
            }
        })
        .collect()
}
