use crate::attributes::PipelineAttributes;
use crate::errors::{Error, Result};
use proc_macro2::TokenStream;
use quote::{quote, quote_spanned};
use syn::{
    DeriveInput, GenericArgument, Type, TypePath, WherePredicate, parse_quote, spanned::Spanned,
};

/// Generates the pipeline methods for a struct with a single field of type `Option<T>`.
///
/// This function supports generic structs by forwarding generics and where clauses,
/// ensuring the inner type `T` is bound by `Clone`.
///
/// Recognized attributes:
/// - `skip = true`: disables pipeline processing, generating stub methods returning `None`.
/// - `timeout = u64`: if set, injects a print statement to log pipeline timeout on method calls.
///
/// # Errors
/// Returns an error if:
/// - The struct does not have exactly one named field.
/// - The single field is not of type `Option<T>` with a concrete generic argument.
/// - The type path in the field's type is malformed.
pub fn pipeline_derive(input: DeriveInput, attrs: &PipelineAttributes) -> Result<TokenStream> {
    let struct_name = &input.ident;

    // Validate that the struct has exactly one named field
    let field = if let syn::Data::Struct(syn::DataStruct {
        fields: syn::Fields::Named(fields),
        ..
    }) = &input.data
    {
        if fields.named.len() == 1 {
            fields.named.first().unwrap()
        } else {
            return Err(Error::spanned(
                &input.ident,
                "Expected a struct with exactly one named field",
            ));
        }
    } else {
        return Err(Error::spanned(
            &input.ident,
            "Expected a struct with named fields",
        ));
    };

    // Extract the identifier of the single named field
    let field_ident = field
        .ident
        .as_ref()
        .ok_or_else(|| Error::spanned(field, "Expected named field with identifier"))?;

    // Extract inner type T from a field with type Option<T>
    let inner_type = if let Type::Path(TypePath { path, .. }) = &field.ty {
        let last_segment = path
            .segments
            .last()
            .ok_or_else(|| Error::spanned(&field.ty, "Malformed type path in field type"))?;
        if last_segment.ident != "Option" {
            return Err(Error::spanned(
                last_segment,
                "Expected field of type Option<T>",
            ));
        }
        if let syn::PathArguments::AngleBracketed(angle_bracketed) = &last_segment.arguments {
            let Some(GenericArgument::Type(ty)) = angle_bracketed.args.first() else {
                return Err(Error::spanned(
                    angle_bracketed,
                    "Expected Option<T> with concrete type",
                ));
            };
            ty
        } else {
            return Err(Error::spanned(
                last_segment,
                "Expected angle-bracketed generic arguments",
            ));
        }
    } else {
        return Err(Error::spanned(
            &field.ty,
            "Expected field of type Option<T>",
        ));
    };

    // Clone generics and add a `T: Clone` where bound to the generics for use in method definitions
    let mut generics = input.generics.clone();
    let clone_bound: WherePredicate = parse_quote! {
        #inner_type: Clone
    };
    if let Some(ref mut wc) = generics.where_clause {
        wc.predicates.push(clone_bound);
    } else {
        generics.where_clause = Some(syn::WhereClause {
            where_token: Default::default(),
            predicates: vec![clone_bound].into_iter().collect(),
        });
    }
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    // If `skip` attribute is true, generate dummy `process` methods that immediately return None
    if attrs.skip {
        return Ok(quote_spanned! { struct_name.span()=>
            impl #impl_generics #struct_name #ty_generics #where_clause {
                /// Always returns None because skip attribute is set.
                pub fn process3<F, G>(&self, _f1: F, _f2: G) -> Option<#inner_type>
                where
                    F: FnOnce(#inner_type) -> Option<#inner_type>,
                    G: FnOnce(#inner_type) -> Option<#inner_type>,
                {
                    None
                }
                /// Always returns None because skip attribute is set.
                pub fn process4<F, G, H>(&self, _f1: F, _f2: G, _f3: H) -> Option<#inner_type>
                where
                    F: FnOnce(#inner_type) -> Option<#inner_type>,
                    G: FnOnce(#inner_type) -> Option<#inner_type>,
                    H: FnOnce(#inner_type) -> Option<#inner_type>,
                {
                    None
                }
            }
        });
    }

    // If `timeout` attribute is set, generate code to print the timeout message on pipeline method calls
    let timeout_code = if let Some(timeout) = attrs.timeout {
        quote! {
            println!("Pipeline timeout set to {} ms", #timeout);
        }
    } else {
        quote! {}
    };

    // Generate the pipeline methods with chained processing steps using Option::and_then
    Ok(quote_spanned! { struct_name.span()=>
        impl #impl_generics #struct_name #ty_generics #where_clause {
            /// Processes the inner Option<T> with two chained closure steps.
            pub fn process3<F, G>(&self, f1: F, f2: G) -> Option<#inner_type>
            where
                F: FnOnce(#inner_type) -> Option<#inner_type>,
                G: FnOnce(#inner_type) -> Option<#inner_type>,
            {
                #timeout_code
                self.#field_ident.as_ref().cloned().and_then(f1).and_then(f2)
            }

            /// Processes the inner Option<T> with three chained closure steps.
            pub fn process4<F, G, H>(&self, f1: F, f2: G, f3: H) -> Option<#inner_type>
            where
                F: FnOnce(#inner_type) -> Option<#inner_type>,
                G: FnOnce(#inner_type) -> Option<#inner_type>,
                H: FnOnce(#inner_type) -> Option<#inner_type>,
            {
                #timeout_code
                self.#field_ident.as_ref().cloned().and_then(f1).and_then(f2).and_then(f3)
            }
        }
    })
}
