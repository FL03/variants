use crate::attrs::{NestedAttr, VariantAttr};
use syn::Attribute;

// AST for the scsys attribute
#[derive(Debug, Default)]
pub struct OuterAttr {
    pub variant: Option<VariantAttr>,
}

impl OuterAttr {
    pub fn set_variant(&mut self, variant: VariantAttr) {
        self.variant = Some(variant);
    }

    // tries to extract the scsys attribute from a list of attributes
    pub fn extract(attrs: &[Attribute]) -> syn::Result<Self> {
        let mut scsys = Self::default();
        for attr in attrs {
            if attr.path().is_ident("variants") {
                attr.parse_nested_meta(|meta| {
                    if let Ok(nested) = NestedAttr::parse_nested(&meta) {
                        match nested {
                            NestedAttr::Variant(inner) => {
                                scsys.set_variant(inner);
                                return Ok(());
                            }
                        }
                    }
                    Err(meta.error("unrecognized scsys attribute"))
                })?;
            }
        }
        Ok(scsys)
    }
}
