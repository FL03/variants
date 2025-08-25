/*
    Appellation: nested <module>
    Contrib: @FL03
*/
use super::VariantAttr;
use syn::Ident;
use syn::parse::{Parse, ParseStream};

/// [`NestedAttr`] is an enumeration of various nested attributes the crate recognizes.
#[derive(Debug)]
pub enum NestedAttr {
    Variant(VariantAttr),
}

impl NestedAttr {
    /// attempts to parse the attribute from the given metadata
    pub fn parse_nested(meta: &syn::meta::ParseNestedMeta<'_>) -> syn::Result<Self> {
        // #[variants(variant(...))]
        if meta.path.is_ident("variant") {
            let attr = VariantAttr::parse_nested(meta)?;
            return Ok(Self::Variant(attr));
        }

        Err(meta.error("unrecognized repr"))
    }
}

impl Parse for NestedAttr {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let ident: Ident = input.parse()?;
        if ident == "variant" {
            dbg!("found variant attribute ");
            let content;
            syn::parenthesized!(content in input);
            // Parse an optional identifier
            let name = if content.is_empty() {
                None
            } else {
                Some(content.parse::<Ident>()?)
            };

            Ok(NestedAttr::Variant(VariantAttr { name }))
        } else {
            Err(syn::Error::new_spanned(ident, "unknown attribute"))
        }
    }
}
