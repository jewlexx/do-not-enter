use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_macro_input, Data, DeriveInput, Fields};

fn macro_error(msg: &str) -> proc_macro::TokenStream {
    quote::quote! {
       compile_error!(#msg)
    }
    .into()
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
        _ => return macro_error("Can only be derived on an enum"),
    };

    let (arms, array_len) = {
        use heck::ToSnakeCase;
        let mut arms = Vec::<TokenStream>::new();

        for var in vars.iter() {
            let mut fn_args = Vec::<TokenStream>::new();
            let mut decl: Vec<TokenStream> = Vec::new();
            let var_name = &var.ident;
            let var_name_str = var_name.to_string().to_snake_case();
            let field_type;

            match var.fields {
                Fields::Unit => field_type = FieldType::Unit,
                Fields::Named(ref fields) => {
                    field_type = FieldType::Named;
                    fields.named.iter().for_each(|f| {
                        let field_name = &f.ident;
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

            let field_braces = match field_type {
                FieldType::Unit => ("", ""),
                FieldType::Named => ("{", "}"),
                FieldType::Unnamed => ("(", ")"),
            };

            let decl_args = quote!(#(#decl),*).to_string();

            let function_body = format_ident!(
                "{}::{}{l}{}{r}",
                name,
                var_name,
                decl_args,
                l = field_braces.0,
                r = field_braces.1
            );

            let arm = quote! {
               pub fn #var_name_str(&self, #(#fn_args),*) {
                    #function_body.to_fg_string()
               }
            };

            arms.push(arm);
        }
        let arms_len = arms.len();

        (arms, arms_len)
    };

    let v = quote! {
        pub trait ColorizeString {
            #(#arms)*
        }
    };

    v.into()
}
