use darling::FromMeta;
use prox::prelude::*;
use quote::quote;

#[derive(Debug, FromMeta)]
pub(crate) struct BuilderParams {
    pub(crate) finish_fn: Option<syn::Ident>,
    pub(crate) builder_type: Option<syn::Ident>,
}

#[derive(Debug, Default)]
pub(crate) struct ItemParams {
    pub(crate) name: Option<syn::Ident>,
    pub(crate) vis: Option<syn::Visibility>,
}

impl FromMeta for ItemParams {
    fn from_meta(meta: &syn::Meta) -> Result<Self> {
        if let syn::Meta::NameValue(meta) = meta {
            let val = &meta.value;
            let ident = syn::parse2(quote!(#val))?;

            return Ok(Self {
                name: Some(ident),
                vis: None,
            });
        }

        #[derive(Debug, FromMeta)]
        struct Full {
            name: Option<syn::Ident>,
            vis: Option<syn::Visibility>,
        }

        let full = Full::from_meta(meta)?;
        let me = Self {
            name: full.name,
            vis: full.vis,
        };

        Ok(me)
    }
}
