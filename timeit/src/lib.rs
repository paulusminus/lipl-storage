use proc_macro::{TokenStream};
use syn::{parse_macro_input, ItemFn};
use quote::{quote};

const ASYNC_MISSING_MSG: &str = "#[time_it]: the `async` keyword is missing from the function declaration";

fn token_stream_with_error(mut tokens: TokenStream, error: syn::Error) -> TokenStream {
    tokens.extend(TokenStream::from(error.into_compile_error()));
    tokens
}


/// Marks async function to report the time it took to execute by means of tracing.
/// 
/// # Example
/// ```rust
/// use timeit::timeit;
/// 
/// #[timeit]
/// async fn test() -> Result<String, std::io::Error> {
///     tokio::fs::read_to_string("Cargo.toml").await
/// }
/// ```
/// 
/// this example is equivalent to the following code
/// 
/// ```rust
/// #[tracing::instrument]
/// async fn test() -> Result<String, std::io::Error> {
///     let now = std::time::Instant::now();
///     let result = async move { tokio::fs::read_to_string("Cargo.toml").await }.await;
///     ::tracing::trace!("Function execution took {:?}", now.elapsed());
///     result
/// }
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

