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

mod timeit;

use proc_macro::{TokenStream};
use syn::{parse_macro_input, ItemFn, AttributeArgs};

/// Marks async function to report the time it took to execute by means of tracing.
/// 
/// # Example
/// 
/// ```
/// use futures::executor::block_on;
/// use futures::future::ready;
/// use timeit::timeit;
/// 
/// #[timeit(level = "info")]
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
    let args = parse_macro_input!(args as AttributeArgs);
    let func = parse_macro_input!(input as ItemFn);

    timeit::expand(args, func)
    .unwrap_or_else(|error| error.to_compile_error())
    .into()
}
