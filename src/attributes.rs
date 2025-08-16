use quote::ToTokens;
use std::fmt;
use syn::{
    Expr, Ident, Result, Token,
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    token::Comma,
};

/// Represents parsed attributes from the `#[pipeline(...)]` attribute.
///
/// Supports recognized keys `skip` and `timeout` with typed values.
/// Unknown keys and optional values are preserved in `others`.
#[derive(Clone, Default)]
pub struct PipelineAttributes {
    /// If true, disables pipeline processing by skipping generation.
    pub skip: bool,
    /// Optional timeout value in milliseconds.
    pub timeout: Option<u64>,
    /// Other unrecognized attribute key-value pairs.
    pub others: Vec<(Ident, Option<Expr>)>,
}

impl fmt::Debug for PipelineAttributes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("PipelineAttributes")
            .field("skip", &self.skip)
            .field("timeout", &self.timeout)
            .field(
                "others",
                &self
                    .others
                    .iter()
                    .map(|(ident, val)| {
                        if let Some(expr) = val {
                            format!("{} = {:?}", ident, expr.to_token_stream())
                        } else {
                            format!("{}", ident)
                        }
                    })
                    .collect::<Vec<_>>(),
            )
            .finish()
    }
}

/// Parses comma-separated key-value pairs inside `#[pipeline(...)]` attribute.
///
/// Recognizes `skip` (boolean) and `timeout` (integer) keys specially.
/// Unknown keys are collected as `others`.
impl Parse for PipelineAttributes {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut attrs = PipelineAttributes::default();

        // Parse comma-separated pairs like `key` or `key = value`
        let pairs = Punctuated::<PipelineAttributePair, Comma>::parse_terminated(input)?;

        for pair in pairs {
            let key_str = pair.key.to_string();
            match key_str.as_str() {
                "skip" => {
                    // Accept either bare `skip` (imply true) or skip = true/false
                    attrs.skip = if let Some(expr) = pair.value {
                        match expr {
                            Expr::Lit(lit) => match &lit.lit {
                                syn::Lit::Bool(b) => b.value,
                                other => {
                                    return Err(syn::Error::new_spanned(
                                        other,
                                        "Expected boolean literal for 'skip'",
                                    ));
                                }
                            },
                            other => {
                                return Err(syn::Error::new_spanned(
                                    other,
                                    "Expected boolean literal for 'skip'",
                                ));
                            }
                        }
                    } else {
                        true // bare `skip` means true
                    };
                }
                "timeout" => {
                    if let Some(expr) = pair.value {
                        match expr {
                            Expr::Lit(lit) => match &lit.lit {
                                syn::Lit::Int(int_lit) => {
                                    attrs.timeout = Some(int_lit.base10_parse()?);
                                }
                                other => {
                                    return Err(syn::Error::new_spanned(
                                        other,
                                        "Expected integer literal for 'timeout'",
                                    ));
                                }
                            },
                            other => {
                                return Err(syn::Error::new_spanned(
                                    other,
                                    "Expected integer literal for 'timeout'",
                                ));
                            }
                        }
                    } else {
                        return Err(syn::Error::new_spanned(
                            pair.key,
                            "'timeout' attribute requires an integer value, e.g. timeout = 1000",
                        ));
                    }
                }
                _ => {
                    // Optional: warn about unknown keys but still collect them
                    let _warn = syn::Error::new_spanned(
                        pair.key.clone(),
                        format!("Unknown pipeline attribute key '{}'", key_str),
                    );
                    // Store unknown attribute key-value pair anyway
                    attrs.others.push((pair.key, pair.value));
                }
            }
        }

        Ok(attrs)
    }
}

/// Represents a single key-value pair in the pipeline attribute.
///
/// Parses `key` or `key = value` pairs.
struct PipelineAttributePair {
    key: Ident,
    value: Option<Expr>,
}

impl Parse for PipelineAttributePair {
    fn parse(input: ParseStream) -> Result<Self> {
        let key: Ident = input.parse()?;
        let value = if input.peek(Token![=]) {
            input.parse::<Token![=]>()?;
            Some(input.parse()?)
        } else {
            None
        };
        Ok(PipelineAttributePair { key, value })
    }
}

/// Parse the `#[pipeline(...)]` attribute from a struct's attributes.
///
/// Returns parsed `PipelineAttributes` or default if attribute not present.
pub fn parse_attributes(input: &syn::DeriveInput) -> Result<PipelineAttributes> {
    for attr in &input.attrs {
        if attr.path().is_ident("pipeline") {
            return attr.parse_args();
        }
    }
    Ok(PipelineAttributes::default())
}
