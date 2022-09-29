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

//! Attribute Procedural Macro for tracing elapsed time

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
/// use timeit::timeit;
/// use tokio::fs::read_to_string;
/// use tokio_test::block_on;
/// 
/// #[timeit]
/// async fn test() -> Result<String, std::io::Error> {
///     read_to_string("Cargo.toml").await
/// }
/// 
/// assert!(block_on(async { test().await }).is_ok());
/// ```
/// 
/// this example is equivalent to the following code
/// 
/// ```
/// use tracing::{instrument, trace};
/// use tokio::fs::read_to_string;
/// use tokio_test::block_on;
/// 
/// #[instrument]
/// async fn test() -> Result<String, std::io::Error> {
///     let now = std::time::Instant::now();
///     let result = async move { read_to_string("Cargo.toml").await }.await;
///     trace!("Function execution took {:?}", now.elapsed());
///     result
/// }
/// 
/// assert!(block_on(async { test().await }).is_ok());
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
                ::tracing::trace!("function execution took {:?}", now.elapsed());
                result
            }
        }
        .into()    
    }
}
