use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn trace(_attr: TokenStream, input: TokenStream) -> TokenStream {
    let syn::ItemFn {
        vis,
        sig,
        mut block,
        ..
    } = syn::parse(input).expect("`trace` macro expects only functions");

    let fn_ident_str = sig.ident.to_string();
    let stmt = syn::parse_quote! {
        println!("{} started", #fn_ident_str);
    };
    block.stmts.insert(0, stmt);

    quote::quote! {
        #vis #sig #block
    }.into()
}
