//派生宏示例
//为结构体自动生成调试输出实现
use proc_macro::TokenStream;
use syn::{Data, DeriveInput, Fields, parse_macro_input};

#[proc_macro_derive(MyDebug)]
pub fn my_debug_derive(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let name = &ast.ident;
    let fields = match &ast.data {
        Data::Struct(data) => match &data.fields {
            Fields::Named(f) => &f.named,
            _ => panic!("Only named fields supported"),
        },
        _ => panic!("Only structs supported"),
    };
    let field_debug = fields.iter().map(|f| {
        let fname = f.ident.as_ref().unwrap();
        quote::quote! {
            parts.push(format!("{}={:?}", stringify!(#fname), &self.#fname));
        }
    });
    let expanded = quote::quote! {
        impl MyDebug for #name {
            fn my_debug(&self) -> String {
                let mut parts = Vec::new();
                #(#field_debug)*
                format!("ssje: {} {{ {} }}", stringify!(#name), parts.join(", "))
            }
        }
    };
    expanded.into()
}
