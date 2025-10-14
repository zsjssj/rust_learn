//!流（Streams）：顺序的 Futures
//参考：https://kaisery.github.io/trpl-zh-cn/ch17-04-streams.html
#![allow(dead_code)]
#![allow(unused_variables)]
use std::{pin::pin, time::Duration};
use trpl::{ReceiverStream, Stream, StreamExt};

pub fn run() {
    // test1();
    // test2();
    test3();
}

/*
  流类似于一种异步形式的迭代器。
  然而， trpl::Receiver 专门等待接收消息，而通用的流 API 适用范围要广泛得多：它以与 Iterator 相同的方式提供下一个元素，但采用异步的方式实现。
  Rust 中迭代器和流的相似性意味着我们实际上可以从任何迭代器上创建流。
*/
fn test1() {
    trpl::run(async {
        let values = 1..101;
        let iter = values.map(|n| n * 2);
        let stream = trpl::stream_from_iter(iter);

        let mut filtered = stream.filter(|value| value % 3 == 0 || value % 5 == 0);
        while let Some(value) = filtered.next().await {
            println!("The value was: {value}");
        }
    });
}

//组合流
fn test2() {
    trpl::run(async {
        let mut messages = pin!(get_messages().timeout(Duration::from_millis(200)));

        while let Some(result) = messages.next().await {
            match result {
                Ok(message) => println!("{message}"),
                Err(reason) => eprintln!("Problem: {reason:?}"),
            }
        }
    })
}
fn get_messages() -> impl Stream<Item = String> {
    let (tx, rx) = trpl::channel();

    trpl::spawn_task(async move {
        let messages = ["a", "b", "c", "d", "e", "f", "g", "h", "i", "j"];

        for (index, message) in messages.into_iter().enumerate() {
            let time_to_sleep = if index % 2 == 0 { 100 } else { 300 };
            trpl::sleep(Duration::from_millis(time_to_sleep)).await;

            if let Err(send_error) = tx.send(format!("Message: '{message}'")) {
                eprintln!("Cannot send message '{message}': {send_error}");
                break;
            }
        }
    });

    ReceiverStream::new(rx)
}

//合并流
pub fn test3() {
    trpl::run(async {
        let messages = get_messages().timeout(Duration::from_millis(200));
        let intervals = get_intervals().map(|count| format!("Interval: {count}")).throttle(Duration::from_millis(100)).timeout(Duration::from_secs(10));
        let merged = messages.merge(intervals).take(20);
        let mut stream = pin!(merged);

        while let Some(result) = stream.next().await {
            match result {
                Ok(message) => println!("{message}"),
                Err(reason) => eprintln!("Problem: {reason:?}"),
            }
        }
    })
}
fn get_intervals() -> impl Stream<Item = u32> {
    let (tx, rx) = trpl::channel();

    trpl::spawn_task(async move {
        let mut count = 0;
        loop {
            trpl::sleep(Duration::from_millis(1)).await;
            count += 1;

            if let Err(send_error) = tx.send(count) {
                eprintln!("Could not send interval {count}: {send_error}");
                break;
            };
        }
    });

    ReceiverStream::new(rx)
}
