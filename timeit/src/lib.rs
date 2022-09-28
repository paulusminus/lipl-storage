use proc_macro::{TokenStream};
use syn::{parse_macro_input, ItemFn};
use quote::quote;

#[proc_macro_attribute]
pub fn timeit(_args: TokenStream, input: TokenStream) -> TokenStream {
    let func = parse_macro_input!(input as ItemFn);

    let vis = func.vis.clone();
    let sig = func.sig.clone();
    let block = func.block;

    let expanded = quote! {
        #vis #sig {
            let now = ::std::time::Instant::now();
            let result = async move #block.await;
            let elapsed_seconds = now.elapsed().as_secs();
            ::tracing::info!("{} seconds elapsed", elapsed_seconds);
            result
        }
    };
    expanded.into()
}

