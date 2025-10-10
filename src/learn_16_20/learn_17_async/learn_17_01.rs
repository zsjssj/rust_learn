//! Futures 和 async 语法
// 参考：https://kaisery.github.io/trpl-zh-cn/ch17-01-futures-and-syntax.html
// future 是一个现在可能还没有准备好但将在未来某个时刻准备好的值。同的概念也出现在很多语言中，有时被称为 “task” 或者 “promise”

#![allow(dead_code)]
#![allow(unused_variables)]
use trpl::Html;

pub fn run() {
    test1()
}
fn test1() {
    println!("learn_17_async run");
}
async fn page_title(url: &str) -> Option<String> {
    let response = trpl::get(url).await;
    let response_text = response.text().await;
    Html::parse(&response_text).select_first("title").map(|title_element| title_element.inner_html())
}
