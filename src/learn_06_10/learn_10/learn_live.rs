#![allow(unused)]
pub fn test() {
    test002();
}

//函数签名中的生命周期标注
fn test001() {
    fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
        if x.len() > y.len() { x } else { y }
    }
    let string1 = String::from("abcd");
    let result;
    {
        let string2 = "xyz"; //string2是字符串slice 。 xyz是字符串字面量，具有 'static 生命周期，即在整个程序运行期间都有效
        // let string2 = String::from("xyz"); // `string2` does not live long enough. borrowed value does not live long enough
        result = longest(string1.as_str(), string2);
    }
    println!("The longest string is {}", result);
}

//结构体定义中的生命周期标注
fn test002() {
    struct ImportantExcerpt<'a> {
        part: &'a str,
    }
    let a1 = String::from("hello world. Some years ago...");
    let a2 = a1.split('.').next().expect("no content");
    let i = ImportantExcerpt { part: a2 };
    println!("ImportantExcerpt: {}", i.part);
}

fn test003() {}
