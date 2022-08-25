use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::DeriveInput;

use crate::macro_error;

pub fn derive_get_set(input: DeriveInput) -> TokenStream {
    let ident = &input.ident;

    let data = match input.data {
        syn::Data::Struct(ref data) => data,
        _ => return macro_error("Can only be derived on a struct"),
    };

    let mut fields_vec = Vec::new();

    match data.fields {
        syn::Fields::Named(ref fields) => {
            for field in fields.named.iter() {
                let name = field.ident.as_ref().unwrap();

                fields_vec.push((name.clone(), name.clone(), field.ty.clone()));
            }
        }
        syn::Fields::Unnamed(ref fields) => {
            for (i, field) in fields.unnamed.iter().enumerate() {
                let fn_name = format_ident!("get_{}", i);
                let name = format_ident!("{}", i);

                fields_vec.push((fn_name.clone(), name, field.ty.clone()));
            }
        }
        syn::Fields::Unit => return macro_error("Struct must have fields"),
    };

    let tokens_vec = fields_vec.iter().map(|x| {
        let set_fn_name = format_ident!("set_{}", x.1);
        let fn_name = &x.0;
        let name = &x.1;
        let fn_type = &x.2;

        quote! {
            pub fn #fn_name(&self) -> &#fn_type {
                &self.#name
            }

            pub fn #set_fn_name(&self, updated: #fn_type) {
                self.#name = updated;
            }
        }
    });

    quote! {
        impl #ident {
            #(#tokens_vec)*
        }
    }
}
