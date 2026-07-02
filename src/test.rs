#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

use std::fmt::format;

#[tokio::main]
pub async fn run() {
    // test_1(true).await;
    test2();
}

async fn test_1(condition: bool) {
    use std::cell::RefCell;
    use tokio::sync::mpsc;

    let (tx, mut rx) = mpsc::channel(100);
    let async_task1 = tokio::spawn(async move {
        let time1 = std::time::Instant::now();
        let val = String::from("hello");
        tx.send(val).await.unwrap();
        for i in 0..5 {
            let val = format!("value: {}", i);
            tx.send(val).await.unwrap();
        }
        tokio::time::sleep(std::time::Duration::from_secs(2)).await;
        for i in 6..10 {
            let val = format!("value: {}", i);
            tx.send(val).await.unwrap();
        }
        let duration = time1.elapsed();
        let msg = "消息发送耗时";
        println!("前面4个字符:{:?}", &msg[..=8]);
        println!("消息发送耗时: {:?}", duration);
    });

    let async_task2 = tokio::spawn(async move {
        while let Some(v) = rx.recv().await {
            println!("recv {}", v);
            tokio::time::sleep(std::time::Duration::from_millis(500)).await;
        }
    });
    async_task1.await.unwrap();
    async_task2.await.unwrap();
}

fn test2() {
    let s1 = String::from("hello");
    let o1 = Some(&s1);
    let s2: Option<String> = o1.cloned();
    println!("s2 = {:?}", s2);
}
