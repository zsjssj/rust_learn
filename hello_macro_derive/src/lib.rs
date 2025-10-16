use proc_macro::TokenStream;
use quote::quote;

#[proc_macro_derive(HelloMacro)]
pub fn hello_macro_derive(input: TokenStream) -> TokenStream {
    // 将 Rust 代码构建成我们可以操作的语法树。
    let ast = syn::parse(input).unwrap();

    // 生成 trait 的实现。
    impl_hello_macro(&ast)
}

fn impl_hello_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let generated = quote! {
        impl HelloMacro for #name {
            fn hello_macro() {
                println!("\x1b[31mHello\x1b[0m, Macro! My name is {}!", stringify!(#name));
            }
        }
    };
    generated.into()
}
