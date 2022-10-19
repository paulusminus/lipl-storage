#![allow(clippy::needless_doctest_main)]
#![warn(
    missing_debug_implementations,
    missing_docs,
    rust_2018_idioms,
    unreachable_pub
)]
#![doc(
    test(
        no_crate_inject,
        attr(
            deny(
                warnings,
                rust_2018_idioms
            ),
            allow(
                dead_code,
                unused_variables
            )
    )
))]

//! This crate contains one Attribute Procedural Macro for tracing elapsed time

use std::collections::HashMap;

use proc_macro::{TokenStream};
use syn::{parse_macro_input, ItemFn, Item, parse::Parser, NestedMeta, spanned::Spanned};
use quote::quote;

type AttributeArgs = syn::punctuated::Punctuated<syn::NestedMeta, syn::Token![,]>;

/// Error message when annotated function is not async
const ASYNC_MISSING_MSG: &str = "#[timeit]: The function declaration is missing the `async` keyword";
/// Error message when annotation has parameters
const FN_HAS_PARAMETERS_MSG: &str = "#[timit]: timeit cannot handle parameters";

fn parse_level(int: &syn::Lit) -> Result<String, syn::Error> {
    if let syn::Lit::Str(lit) = int {
        let level = lit.value().to_lowercase();
        if ["trace", "debug", "info", "warn", "error"].contains(&level.as_str()) {
            Ok(level)
        }
        else {
            Err(
                syn::Error::new(lit.span(), format!("parameter has invalid value"))
            )
        }
    }
    else {
        Err(
            syn::Error::new(int.span(), "Literal missing")
        )
    }
}

fn is_async(tokens: &ItemFn) -> Result<(), syn::Error> {
    tokens.sig.asyncness.ok_or(
        syn::Error::new(
            tokens.sig.span(),
            ASYNC_MISSING_MSG,
        )
    )
    .map(|_| {})
}

fn build_config(args: TokenStream) -> Result<String, syn::Error> {
    AttributeArgs::parse_terminated.parse(args.clone())
    .map_err(
        |_| syn::Error::new_spanned(
            Item::Verbatim(args.into()),
            "timeit: Cannot parse argument",
        )
    )
    .and_then(|args|
        args
        .first()
        .ok_or(
            syn::Error::new(
                args.span(),
                FN_HAS_PARAMETERS_MSG
            )
         )
         .and_then(|lit| 
            if let NestedMeta::Lit(level) = lit {
                parse_level(level)
            }
            else {
                Err(
                    syn::Error::new(lit.span(), "timeit: argument is not a literal") 
                )
            }
        )
    )
}

/// Report error to the compiler
fn token_stream_with_error(mut tokens: TokenStream, error: syn::Error) -> TokenStream {
    tokens.extend(TokenStream::from(error.into_compile_error()));
    tokens
}

/// Marks async function to report the time it took to execute by means of tracing.
/// 
/// # Example
/// 
/// ```
/// use futures::executor::block_on;
/// use futures::future::ready;
/// use timeit::timeit;
/// 
/// #[timeit("info")]
/// async fn test() -> &'static str {
///     ready("Cargo.toml").await
/// }
/// 
/// assert_eq!(block_on(test()), "Cargo.toml");
/// ```
/// 
/// this example is equivalent to the following code
/// 
/// ```
/// use futures::executor::block_on;
/// use futures::future::ready;
/// use tracing::{event, instrument, Level};
/// 
/// #[instrument]
/// async fn test() -> &'static str {
///     let now = std::time::Instant::now();
///     let result = async move { ready("Cargo.toml").await }.await;
///     event!(Level::INFO, elapsed_microseconds = now.elapsed().as_micros());
///     result
/// }
/// 
/// assert_eq!(block_on(test()), "Cargo.toml");
/// ```
#[proc_macro_attribute]
pub fn timeit(args: TokenStream, input: TokenStream) -> TokenStream {
    let fn_tokens = input.clone();
    let func = parse_macro_input!(fn_tokens as ItemFn);

    match is_async(&func).and_then(|_| build_config(args)) {
        Ok(level) => {
            let vis = func.vis.clone();
            let sig = func.sig.clone();
            let block = func.block;

            let hm: HashMap<String, proc_macro2::TokenStream> = HashMap::from_iter([
                ("trace".to_owned(), quote! { ::tracing::Level::TRACE }),
                ("debug".to_owned(), quote! { ::tracing::Level::DEBUG }),
                ("info".to_owned(), quote! { ::tracing::Level::INFO }),
                ("warn".to_owned(), quote! { ::tracing::Level::WARN }),
                ("error".to_owned(), quote! { ::tracing::Level::ERROR }),
            ]);
            let l = hm.get(&level).unwrap();

            quote! {
                #[::tracing::instrument]
                #vis #sig {
                    let now = ::std::time::Instant::now();
                    let result = async move #block.await;
                    ::tracing::event!(#l, elapsed_microseconds = now.elapsed().as_micros());
                    result
                }
            }
            .into()        
        },
        Err(error) => {
            token_stream_with_error(
                input,
                error,
            )    
        }
    }

    

    // if !args.is_empty() {
    //     token_stream_with_error(
    //         input,
    //         syn::Error::new_spanned(
    //             Item::Verbatim(args.into()),
    //             FN_HAS_PARAMETERS_MSG,
    //         ),
    //     )
    // }
    // else if func.sig.asyncness.is_none() {
    //     token_stream_with_error(
    //         input,
    //         syn::Error::new_spanned(
    //             func.sig.fn_token,
    //             ASYNC_MISSING_MSG,
    //         ),
    //     )
    // }
    // else {
    //     let vis = func.vis.clone();
    //     let sig = func.sig.clone();
    //     let block = func.block;
    //     quote! {
    //         #[::tracing::instrument]
    //         #vis #sig {
    //             let now = ::std::time::Instant::now();
    //             let result = async move #block.await;
    //             ::tracing::event!(::tracing::Level::INFO, elapsed_micros = now.elapsed().as_micros());
    //             result
    //         }
    //     }
    //     .into()    
    // }
}
