use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_macro_input, Data, DeriveInput, Fields};

fn macro_error(msg: &str) -> proc_macro::TokenStream {
    quote::quote! {
       compile_error!(#msg)
    }
    .into()
}

#[proc_macro_derive(ImplColorus)]
pub fn derive_impl_colours(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);

    let name = &ast.ident;
    let vars = match &ast.data {
        Data::Enum(v) => &v.variants,
        _ => return macro_error("Can only be derived on an enum"),
    };

    let (array_arms, array_len) = {
        use heck::ToSnakeCase;
        let mut arms = Vec::<TokenStream>::new();

        for var in vars.iter() {
            let mut fn_args = Vec::<TokenStream>::new();
            let var_name = &var.ident;
            let var_name_str = var_name.to_string();

            match var.fields {
                Fields::Unit => (),
                Fields::Named(ref fields) => fields.named.iter().for_each(|f| {
                    let field_name = &f.ident;
                    let field_type = &f.ty;
                    fn_args.push(quote! {
                        #field_name: #field_type,
                    });
                }),
                Fields::Unnamed(ref fields) => {
                    fields.unnamed.iter().enumerate().for_each(|(i, f)| {
                        let field_type = &f.ty;
                        let field_name = format_ident!("a{}", i);
                        fn_args.push(quote! {
                            #field_name: #field_type,
                        });
                    })
                }
            };

            let arm = quote! { #name::#var_name };

            arms.push(arm);
        }
        let arms_len = arms.len();

        (arms, arms_len)
    };

    let str_vec = {
        let mut arms = Vec::<String>::new();

        for var in vars.iter() {
            let var_name = &var.ident;

            match var.fields {
                Fields::Unit => (),
                _ => return macro_error("Enum cannot have arguments"),
            };

            arms.push(var_name.to_string());
        }

        arms
    };

    let str_array = {
        str_vec
            .iter()
            .map(|x| {
                quote! { #x }
            })
            .collect::<Vec<_>>()
    };

    let str_snake_array = {
        use heck::ToSnakeCase;
        str_vec
            .iter()
            .map(|x| {
                let snake = x.to_snake_case();

                quote! { #snake }
            })
            .collect::<Vec<_>>()
    };

    let v = quote! {
        impl #name {
            pub const fn to_array() -> [#name; #array_len] {
                [#(#array_arms),*]
            }

            pub const fn to_str_array() -> [&'static str; #array_len] {
                [#(#str_array),*]
            }

            pub const fn to_snake_array() -> [&'static str; #array_len] {
                [#(#str_snake_array),*]
            }
        }
    };

    v.into()
}
