use proc_macro::TokenStream;
// use proc_macro2::TokenStream as TokenStream2;

#[proc_macro_attribute]
pub fn trace(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let syn::ItemFn {
        attrs: _attrs,
        vis,
        sig,
        mut block,
    } = syn::parse(item).expect("`trace` macro supports only functions");

    let fn_ident = sig.ident.to_string();

    block.stmts.insert(0, syn::parse_quote!{
        println!("{} start", #fn_ident);
    });


    quote::quote! {
        #vis #sig #block
    }.into()
}
