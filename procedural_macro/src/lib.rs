use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{parse_macro_input, ItemFn};

#[proc_macro_attribute]
pub fn function_to_string(_attr: TokenStream, item: TokenStream) -> TokenStream {
    // Parse the input function
    let input_fn = parse_macro_input!(item as ItemFn);

    // Create string representation of the function
    let function_str = format!("{}", input_fn.to_token_stream());

    // Define a new function with the same name as the input function
    let fn_ident = input_fn.sig.ident;
    let fn_inputs = input_fn.sig.inputs;
    let fn_generics = input_fn.sig.generics;

    // Generate output function
    let output = quote! {
        pub fn #fn_ident #fn_generics(#fn_inputs) -> &'static str {
            #function_str
        }
    };

    output.into()
}
