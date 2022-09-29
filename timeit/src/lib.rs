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

use proc_macro::TokenStream;
use syn::{parse_macro_input, ItemFn};
use quote::quote;

// Error message when annotated function is not async
const ASYNC_MISSING_MSG: &str = "#[time_it]: the `async` keyword is missing from the function declaration";

// Report error to the compiler
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
/// #[timeit]
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
/// use tracing::{instrument, trace};
/// 
/// #[instrument]
/// async fn test() -> &'static str {
///     let now = std::time::Instant::now();
///     let result = async move { ready("Cargo.toml").await }.await;
///     trace!("Execution took {:?}", now.elapsed());
///     result
/// }
/// 
/// assert_eq!(block_on(test()), "Cargo.toml");
/// ```
#[proc_macro_attribute]
pub fn timeit(_args: TokenStream, input: TokenStream) -> TokenStream {
    let fn_tokens = input.clone();
    let func = parse_macro_input!(fn_tokens as ItemFn);

    if func.sig.asyncness.is_none() {
        token_stream_with_error(
            input,
            syn::Error::new_spanned(
                func.sig.fn_token,
                ASYNC_MISSING_MSG,
            ),
        )
    }
    else {
        let vis = func.vis.clone();
        let sig = func.sig.clone();
        let block = func.block;
        quote! {
            #[::tracing::instrument]
            #vis #sig {
                let now = ::std::time::Instant::now();
                let result = async move #block.await;
                ::tracing::trace!("Execution took {:?}", now.elapsed());
                result
            }
        }
        .into()    
    }
}
