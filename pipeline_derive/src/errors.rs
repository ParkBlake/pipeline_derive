use proc_macro2::{Span, TokenStream};
use quote::ToTokens;
use syn::Error as SynError;

/// Wrapper type for syn::Error providing convenient constructors and conversions.
#[derive(Debug)]
pub struct Error(SynError);

impl Error {
    /// Create a new error with a message and a span pointing to the error location.
    pub fn new<T: ToString>(msg: T, span: Span) -> Self {
        Self(SynError::new(span, msg.to_string()))
    }

    /// Create a new error with a message and tokens to which the error will be spanned.
    pub fn spanned<T: ToString>(tokens: impl ToTokens, msg: T) -> Self {
        Self(SynError::new_spanned(tokens, msg.to_string()))
    }

    /// Get a reference to the inner syn::Error.
    pub fn as_syn(&self) -> &SynError {
        &self.0
    }

    /// Convert the error into a compile error token stream by reference.
    pub fn to_compile_error(&self) -> TokenStream {
        self.0.to_compile_error()
    }

    /// Convert the error into a compile error token stream, consuming self.
    pub fn into_compile_error(self) -> TokenStream {
        self.0.to_compile_error()
    }
}

impl From<Error> for SynError {
    fn from(err: Error) -> Self {
        err.0
    }
}

impl From<SynError> for Error {
    fn from(err: SynError) -> Self {
        Self(err)
    }
}

/// Type alias for a Result with this crate's Error type.
pub type Result<T> = std::result::Result<T, Error>;
