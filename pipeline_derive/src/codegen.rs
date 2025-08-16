use crate::attributes::{self, PipelineAttributes};
use crate::errors::Result;
use proc_macro2::TokenStream;
use syn::DeriveInput;

use crate::pipeline;

/// Entry point for pipeline code generation.
///
/// Parses pipeline attributes from the input, then delegates to `pipeline::pipeline_derive`.
pub fn pipeline_derive(input: DeriveInput) -> Result<TokenStream> {
    // Parse pipeline attributes from struct attributes (e.g. #[pipeline(skip = true)])
    let attrs: PipelineAttributes = attributes::parse_attributes(&input)?;

    // Delegate to main pipeline derive logic with parsed attributes
    pipeline::pipeline_derive(input, &attrs)
}
