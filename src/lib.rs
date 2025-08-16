extern crate proc_macro;

use proc_macro::TokenStream;
use syn::parse_macro_input;

mod attributes;
mod codegen;
mod errors;
mod pipeline;

/// Derive macro implementing `Pipeline` for structs with `Option<T>` fields.
///
/// Parses the input syntax tree and any optional `#[pipeline(...)]` attributes,
/// then generates pipeline methods that enable monadic chaining of closures
/// over the optional value.
///
/// Returns compiler errors if parsing or generation fail.
#[proc_macro_derive(Pipeline, attributes(pipeline))]
pub fn pipeline_derive_macro(input: TokenStream) -> TokenStream {
    // Parse the input tokens into a DeriveInput syntax tree
    let input = parse_macro_input!(input as syn::DeriveInput);

    // Parse custom pipeline attributes, defaulting on error by emitting compile error
    let attrs = match attributes::parse_attributes(&input) {
        Ok(a) => a,
        Err(err) => return err.to_compile_error().into(),
    };

    // Generate pipeline implementation, or emit compile error if generation fails
    match pipeline::pipeline_derive(input, &attrs) {
        Ok(tokens) => tokens.into(),
        Err(err) => err.to_compile_error().into(),
    }
}
