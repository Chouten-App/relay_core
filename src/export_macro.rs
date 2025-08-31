use proc_macro2::TokenStream;
use crate::alloc::string::ToString;
use quote::quote;
use syn::{parse_macro_input, ItemFn};

#[proc_macro_attribute]
pub fn wasm_export(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemFn);
    let sig = &input.sig;
    let block = &input.block;
    let fn_name = &sig.ident;
    let output = &sig.output;

    let fallback = match output {
        syn::ReturnType::Default => quote! {},
        syn::ReturnType::Type(_, ty) => {
            if quote!(#ty).to_string().contains("*const") {
                quote! { core::ptr::null() }
            } else {
                quote! { 0 }
            }
        }
    };

    let expanded = quote! {
        #[no_mangle]
        #sig {
            let result = ::core::panic::catch_unwind(|| #block);
            match result {
                Ok(val) => val,
                Err(_) => {
                    $crate::log!("Rust panic in {}", stringify!(#fn_name));
                    #fallback
                }
            }
        }
    };
    TokenStream::from(expanded)
}