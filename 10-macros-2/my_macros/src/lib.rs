use proc_macro::TokenStream;
use quote::quote;
use syn::{DeriveInput, Expr, parse_macro_input};

#[proc_macro]
pub fn say_hello(input: TokenStream) -> TokenStream {
    let expr = parse_macro_input!(input as Expr);

    let expanded = quote! {
        println!("Hello, {}!", stringify!(#expr));
    };

    TokenStream::from(expanded)
}

#[proc_macro_attribute]
pub fn log_call(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut func = parse_macro_input!(item as syn::ItemFn);

    let fn_name = &func.sig.ident;

    let original_fn_body = func.block.clone();

    func.block = syn::parse_quote! {
        {
            println!("Calling {}", stringify!(#fn_name));

            let result = #original_fn_body;

            println!("Exiting {}", stringify!(#fn_name));

            result
        }

    };

    TokenStream::from(quote! {
        #func
    })
}

#[proc_macro_derive(SimpleDebug)]
pub fn simple_debug(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let struct_name = &input.ident;

    let fields = match &input.data {
        syn::Data::Struct(s) => &s.fields,
        _ => panic!("Only structs are supported"),
    };

    let expanded = match fields {
        syn::Fields::Named(fields_named) => {
            let field_names = fields_named.named.iter().map(|field| &field.ident);
            quote! {
                impl std::fmt::Debug for #struct_name {
                    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        let mut debug_info = f.debug_struct(stringify!(#struct_name));

                        #(
                            debug_info.field(stringify!(#field_names), &self.#field_names);
                        )*

                        debug_info.finish()
                    }
                }
            }
        }
        syn::Fields::Unnamed(fields_unnamed) => {
            let fields_len = fields_unnamed.unnamed.len();
            let indice = (0..fields_len).map(|i| syn::Index::from(i));
            quote! {
                impl std::fmt::Debug for #struct_name {
                    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        let mut debug_info = f.debug_tuple(stringify!(#struct_name));

                        #(
                            debug_info.field(&self.#indice);
                        )*

                        debug_info.finish()
                    }
                }
            }
        },
        syn::Fields::Unit => {
            quote! {
                impl std::fmt::Debug for #struct_name {
                    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        f.write_str(stringify!(#struct_name))
                    }
                }
            }
        },
    };

    expanded.into()
}
