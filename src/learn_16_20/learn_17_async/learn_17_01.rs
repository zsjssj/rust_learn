//! Futures 和 async 语法
// 参考：https://kaisery.github.io/trpl-zh-cn/ch17-01-futures-and-syntax.html
// future 是一个现在可能还没有准备好但将在未来某个时刻准备好的值。同的概念也出现在很多语言中，有时被称为 “task” 或者 “promise”

#![allow(dead_code)]
#![allow(unused_variables)]
use trpl::{Either, Html};

pub fn run() {
    // test1();
    test2();
}
fn test1() {
    let args: Vec<String> = std::env::args().collect();
    trpl::run(async {
        let url = &args[1];
        match page_title(url).await {
            Some(title) => println!("The title for {url} was {title}"),
            None => println!("{url} had no title"),
        }
    });
}
//等同于 page_title01
async fn page_title(url: &str) -> Option<String> {
    let text: String = trpl::get(url).await.text().await;
    Html::parse(&text).select_first("title").map(|title_element| title_element.inner_html())
}

fn page_title01(url: &str) -> impl Future<Output = Option<String>> {
    async move {
        let text: String = trpl::get(url).await.text().await;
        Html::parse(&text).select_first("title").map(|title_element| title_element.inner_html())
    }
}

//让两个 URL 相互竞争
fn test2() {
    let args: Vec<String> = std::env::args().collect();

    trpl::run(async {
        let title_fut_1 = page_title_02(&args[1]);
        let title_fut_2 = page_title_02(&args[2]);

        let (url, maybe_title) = match trpl::race(title_fut_1, title_fut_2).await {
            Either::Left(left) => left,
            Either::Right(right) => right,
        };

        println!("{url} \x1b[36mreturned first\x1b[0m");
        match maybe_title {
            Some(title) => println!("\x1b[32mIts page title is:\x1b[0m '{title}'"),
            None => println!("Its title could not be parsed."),
        }
    })
}
async fn page_title_02(url: &str) -> (&str, Option<String>) {
    let text = trpl::get(url).await.text().await;
    let title = Html::parse(&text).select_first("title").map(|title| title.inner_html());
    (url, title)
}
