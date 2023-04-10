use std::str::FromStr;

use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_macro_input, Data, DeriveInput, Fields};

fn macro_error(msg: &str) -> TokenStream {
    quote::quote! {
       compile_error!(#msg)
    }
}

enum FieldType {
    Unit,
    Named,
    Unnamed,
}

#[proc_macro_derive(ImplColorus)]
pub fn derive_impl_colours(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);

    let name = &ast.ident;
    let vars = match &ast.data {
        Data::Enum(v) => &v.variants,
        _ => return macro_error("Can only be derived on an enum").into(),
    };

    let arms = {
        use heck::ToSnakeCase;
        let mut arms = Vec::<TokenStream>::new();

        for var in vars.iter() {
            let mut fn_args = Vec::<TokenStream>::new();
            let mut decl: Vec<TokenStream> = Vec::new();
            let var_name = &var.ident;
            let var_name_str =
                TokenStream::from_str(var_name.to_string().to_snake_case().as_str()).unwrap();
            let field_type;

            match var.fields {
                Fields::Unit => field_type = FieldType::Unit,
                Fields::Named(ref fields) => {
                    field_type = FieldType::Named;
                    fields.named.iter().for_each(|f| {
                        let field_name = f.ident.as_ref().unwrap();
                        let field_type = &f.ty;
                        fn_args.push(quote! {
                            #field_name: #field_type
                        });
                        decl.push(quote!(#field_name));
                    });
                }
                Fields::Unnamed(ref fields) => {
                    field_type = FieldType::Unnamed;
                    fields.unnamed.iter().enumerate().for_each(|(i, f)| {
                        let field_type = &f.ty;
                        let field_name = format_ident!("a{}", i);
                        fn_args.push(quote! {
                            #field_name: #field_type
                        });
                        decl.push(quote!(#field_name));
                    })
                }
            };

            let (f_l, f_r) = match field_type {
                FieldType::Unit => ("", ""),
                FieldType::Named => ("{", "}"),
                FieldType::Unnamed => ("(", ")"),
            };

            let decl_args = quote!(#(#decl),*).to_string();

            let function_body = format!("{name}::{var_name}{f_l}{decl_args}{f_r}.to_fg_str()");
            let fun_bod = TokenStream::from_str(function_body.as_str()).unwrap();

            let arm = quote! {
                fn #var_name_str(&self, #(#fn_args),*) -> String
                where
                    Self: core::fmt::Display,
                {
                    format!("{s}{}{e}", self, s = #fun_bod, e = Color::Reset)
                }
            };

            arms.push(arm);
        }

        arms
    };

    let v = quote! {
        pub trait ColorizeString {
            #(#arms)*
        }

        impl<'a> ColorizeString for &'a str {}
        impl ColorizeString for String {}
    };

    v.into()
}
