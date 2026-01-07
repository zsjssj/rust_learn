//属性宏、派生宏、过程宏的实现
use proc_macro::TokenStream;

// 最简单的属性宏
#[proc_macro_attribute]
pub fn hello(_attr: TokenStream, item: TokenStream) -> TokenStream {
    println!("hello attribute macro");
    item
}
// 带参数的属性宏:attr 参数可以是任意 TokenStream
#[proc_macro_attribute]
pub fn my_attr(attr: TokenStream, item: TokenStream) -> TokenStream {
    println!("my_attr attribute macro: attr = {}", attr);
    item
}

//用 syn 解析 item（工程核心）
use syn::{ItemFn, parse_macro_input};
#[proc_macro_attribute]
pub fn trace(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemFn);
    let name = &input.sig.ident;
    let expanded = quote::quote! {
        #input
        fn print_name() {
            println!("function: {}", stringify!(#name));
        }
    };
    expanded.into()
}

//
// 宏实现思路: 拿到函数签名,拿到函数体,用闭包包裹原 body,插入前后逻辑
#[proc_macro_attribute]
pub fn log(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut f = parse_macro_input!(item as ItemFn);
    let name = &f.sig.ident;
    let block = &f.block;
    f.block = Box::new(syn::parse_quote!({
        println!("enter {}", stringify!(#name));
        let result = (|| #block)();
        println!("exit {}", stringify!(#name));
        result
    }));
    quote::quote!(#f).into()
}

// 解析属性参数（attr）
use syn::meta::ParseNestedMeta;
#[proc_macro_attribute]
pub fn log2(attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut level: Option<i32> = None;
    let mut f = parse_macro_input!(item as ItemFn);
    let name = &f.sig.ident;
    let block = &f.block;
    let parser = syn::meta::parser(|meta: ParseNestedMeta| {
        if meta.path.is_ident("level") {
            let lit: syn::LitInt = meta.value()?.parse()?;
            level = Some(lit.base10_parse()?);
            Ok(())
        } else {
            Err(meta.error("unsupported attribute"))
        }
    });
    parse_macro_input!(attr with parser);
    f.block = Box::new(syn::parse_quote!({
        println!("enter {} level={:?}", stringify!(#name), #level);
        let result = (|| #block)();
        println!("exit {}", stringify!(#name));
        result
    }));
    quote::quote!(#f).into()
}
