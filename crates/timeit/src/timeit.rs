use std::collections::HashMap;

use proc_macro2::TokenStream;
use quote::quote;
use syn::{AttributeArgs, Error, ItemFn, NestedMeta, Meta, Lit, LitStr};

fn trace() -> TokenStream { quote!{ ::tracing::Level::TRACE } } 
fn debug() -> TokenStream { quote!{ ::tracing::Level::DEBUG } } 
fn info()  -> TokenStream { quote!{ ::tracing::Level::INFO } } 
fn warn()  -> TokenStream { quote!{ ::tracing::Level::WARN } }
fn error() -> TokenStream { quote!{ ::tracing::Level::ERROR } }

pub(crate) fn expand(args: AttributeArgs, input: ItemFn) -> Result<TokenStream, Error> {
    // Default level = tracing::Level::INFO
    let mut level = info();

    for arg in args {
        match arg {
            NestedMeta::Meta(Meta::NameValue(nv)) => {
                if nv.path.is_ident("level") {
                    if let Lit::Str(s) = &nv.lit {
                        level = create_level(s)?;
                    }
                }
            }
            _ => { 
                return Err(
                    Error::new_spanned(
                        arg,
                        "unknown attribute"
                    )
                );
            }
        }
    }

    Ok(
        create_timeit(input, level)
    )
}

fn create_level(level: &LitStr) -> Result<TokenStream, syn::Error> {
    HashMap::from([
        ("trace", trace()),
        ("debug", debug()),
        ("info",  info()),
        ("warn",  warn()),
        ("error", error()),
    ])
    .get(level.value().to_lowercase().as_str())
    .cloned()
    .ok_or_else(
        || syn::Error::new_spanned(level, "Invalid level")
    )
}

fn create_timeit(func: ItemFn, level: TokenStream) -> TokenStream {
    let block = func.block;
    let visibility = func.vis.clone();
    let signature = func.sig.clone();

    let result_block = match func.sig.asyncness {
        Some(_) => quote! { let result = async move #block.await; },
        None => quote! { let result = #block; }
    };
    
    quote! {
        #[::tracing::instrument]
        #visibility #signature {
            let now = ::std::time::Instant::now();
            #result_block
            ::tracing::event!(#level, elapsed_microseconds = now.elapsed().as_micros());
            result
        }
    }
}