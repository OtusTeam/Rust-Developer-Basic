use proc_macro::TokenStream;
use quote::quote;
use syn::{parse::Parse, punctuated::Punctuated};

struct Input {
    key_values: Punctuated<KeyValuePair, syn::Token![,]>,
}

impl Parse for Input {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(Self {
            key_values: Punctuated::parse_terminated(input)?,
        })
    }
}

struct KeyValuePair {
    key: syn::Expr,
    _colon_token: syn::Token![:],
    value: syn::Expr,
}

impl Parse for KeyValuePair {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(KeyValuePair {
            key: input.parse()?,
            _colon_token: input.parse()?,
            value: input.parse()?,
        })
    }
}

#[proc_macro]
pub fn map(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as Input);
    let key_values = input.key_values.iter().map(|kv| {
        let key = &kv.key;
        let value = &kv.value;
        quote! { #key, #value }
    });

    quote! {{
        let mut map = std::collections::BTreeMap::new();
        #(
            map.insert(#key_values);
        )*
        map

    }}
    .into()
}
