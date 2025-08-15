extern crate proc_macro;

use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{
    Data, DeriveInput, Error, Fields, GenericArgument, PathArguments, Type, parse_macro_input,
};

/// Extracts the inner type `T` from an `Option<T>`.
/// Returns `None` if the type is not an `Option`.
fn extract_option_inner(ty: &Type) -> Option<&Type> {
    if let Type::Path(type_path) = ty
        && let Some(segment) = type_path.path.segments.last()
        && segment.ident == "Option"
        && let PathArguments::AngleBracketed(args) = &segment.arguments
        && let Some(GenericArgument::Type(inner_ty)) = args.args.first()
    {
        Some(inner_ty)
    } else {
        None
    }
}

/// Generates a pipeline method with `n` steps.
/// Each step is a closure: `FnOnce(Input) -> Option<Input>`.
/// The pipeline chains these closures using `and_then`,
/// immediately returning `None` if any step fails.
fn gen_process_fn(
    n: usize,
    field_ident: &syn::Ident,
    input_type: &Type,
) -> proc_macro2::TokenStream {
    // Create generic closure parameter names: PF0, PF1, ..., PF{n-2}
    let closures = (0..n - 1)
        .map(|i| format_ident!("PF{}", i))
        .collect::<Vec<_>>();

    // Generate parameter names for each step: step1, step2, ...
    let param_names = (0..n - 1)
        .map(|i| format_ident!("step{}", i + 1))
        .collect::<Vec<_>>();

    // Define method parameters with closure types
    let params = closures
        .iter()
        .zip(&param_names)
        .map(|(closure, name)| quote! { #name: #closure });

    // Add trait bounds: each closure must be FnOnce(Input) -> Option<Input>
    let bounds = closures.iter().map(|closure| {
        quote! { #closure: FnOnce(#input_type) -> Option<#input_type> }
    });

    // Compose chained `.and_then` calls on the Option field
    let chain = param_names.iter().fold(
        quote! { self.#field_ident.as_ref().cloned() },
        |acc, name| quote! { #acc.and_then(#name) },
    );

    let fn_name = format_ident!("process{}", n);

    quote! {
        /// Executes a pipeline with `n - 1` processing steps.
        /// Each step consumes the input and returns an `Option`.
        /// The pipeline short-circuits and returns `None` immediately if any step returns `None`.
        pub fn #fn_name<
            #( #closures, )*
        >(
            &self,
            #( #params ),*
        ) -> Option<#input_type>
        where
            #( #bounds ),*
        {
            #chain
        }
    }
}

#[proc_macro_derive(Pipeline)]
pub fn pipeline_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    // Only structs with exactly one named field are supported
    let (field_ident, field_type) = match &input.data {
        Data::Struct(data_struct) => match &data_struct.fields {
            Fields::Named(fields_named) if fields_named.named.len() == 1 => {
                let field = &fields_named.named[0];
                (field.ident.as_ref().unwrap(), &field.ty)
            }
            _ => {
                return Error::new_spanned(
                    &input.ident,
                    "Pipeline derive only supports structs with exactly one named field",
                )
                .to_compile_error()
                .into();
            }
        },
        _ => {
            return Error::new_spanned(&input.ident, "Pipeline derive only supports structs")
                .to_compile_error()
                .into();
        }
    };

    // Confirm the field is Option<T> and extract T
    let inner_type = extract_option_inner(field_type).unwrap_or_else(|| {
        panic!(
            "Expected the field `{}` to be of type Option<T>",
            field_ident
        )
    });

    // Generate pipeline methods with 3 and 4 steps
    let process3 = gen_process_fn(3, field_ident, inner_type);
    let process4 = gen_process_fn(4, field_ident, inner_type);

    let generics = &input.generics;
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
    let name = &input.ident;

    let expanded = quote! {
        impl #impl_generics #name #ty_generics #where_clause {
            #process3
            #process4
        }
    };

    TokenStream::from(expanded)
}
