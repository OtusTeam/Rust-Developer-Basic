use proc_macro::TokenStream;

#[proc_macro_derive(TestValue)]
pub fn derive_test_value(input: TokenStream) -> TokenStream {
    let input: syn::DeriveInput = syn::parse(input).expect("Failed to parse as DeriveInput");
    let ident = input.ident;

    let syn::Data::Struct(data_struct) = input.data else {
        panic!("Only structs are supported");
    };

    let syn::Fields::Named(named_fields) = data_struct.fields else {
        panic!("Only named fields are supported");
    };

    let mut field_constructors = syn::punctuated::Punctuated::<syn::FieldValue, syn::Token![,]>::default();

    for field in named_fields.named {
        let field_ident = field.ident.unwrap();
        let field_type = field.ty;
        field_constructors.push(syn::parse_quote! {
            #field_ident: <#field_type as ::test_value::TestValue>::test_value()
        });
    }

    quote::quote! {
        impl ::test_value::TestValue for #ident {
            fn test_value() -> Self {
                Self {
                    #field_constructors
                }
            }
        }
    }.into()
}
