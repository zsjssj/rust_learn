//!并发与 async
//参考：https://kaisery.github.io/trpl-zh-cn/ch17-02-concurrency-with-async.html
#![allow(dead_code)]
#![allow(unused_variables)]
use std::time::Duration;

pub fn run() {
    // test1();
    // test2();
    // test3();
    // test31();
    // test32();
    test33();
}

fn test1() {
    trpl::run(async {
        let handle = trpl::spawn_task(async {
            for i in 1..10 {
                println!("hi number {i} from the first task!");
                trpl::sleep(Duration::from_millis(500)).await;
            }
        });

        for i in 1..5 {
            println!("hi number {i} from the second task!");
            trpl::sleep(Duration::from_millis(500)).await;
        }
        handle.await.unwrap();
    });
}

//等待多个任务完成
fn test2() {
    trpl::run(async {
        let fut1 = async {
            for i in 1..10 {
                println!("hi number {i} from the first task!");
                trpl::sleep(Duration::from_millis(500)).await;
            }
        };

        let fut2 = async {
            for i in 1..5 {
                println!("hi number {i} from the second task!");
                trpl::sleep(Duration::from_millis(500)).await;
            }
        };

        trpl::join(fut1, fut2).await;
    });
}

//消息传递
/*
使用了 trpl::channel，一个第十六章用于线程的多生产者、单消费者信道 API 的异步版本。
异步版本的 API 与基于线程的版本只有一点微小的区别：
  它使用一个可变的而不是不可变的 rx，并且它的 recv 方法产生一个需要 await 的 future 而不是直接返回值。
*/
fn test3() {
    trpl::run(async {
        let (tx, mut rx) = trpl::channel();

        let val = String::from("hi");
        tx.send(val).unwrap();

        let received = rx.recv().await.unwrap();
        println!("Got: {received}");
    })
}
fn test31() {
    trpl::run(async {
        let (tx, mut rx) = trpl::channel();
        let vals = vec![String::from("hi"), String::from("from"), String::from("the"), String::from("future")];

        for val in vals {
            tx.send(val).unwrap();
            trpl::sleep(Duration::from_millis(500)).await;
        }

        while let Some(value) = rx.recv().await {
            println!("received '{value}'");
        }
    })
}

fn test32() {
    trpl::run(async {
        let (tx, mut rx) = trpl::channel();
        //如果不使用move,程序仍然永远也不会退出，
        /*
        1. trpl::join 返回的 future 只会完成一次，即传递的 两个 future 都完成的时候。
        2. tx future 在发送 vals 中最后一条消息之后的休眠结束后立刻完成。
        3. rx future 直到 while let 循环结束之前都不会完成。
        4. 当信道的另一端关闭后 await rx.recv 将只会返回 None。
        5. 信道只有在调用 rx.close 或者发送端 tx 被丢弃时才会关闭。
        6. 我们没有在任何地方调用 rx.close，并且 tx 直到传递给 trpl::run 的最外层异步代码块结束前都不会被丢弃。
        7. 代码块不能结束是因为它阻塞在了等待 trpl::join 完成，这就又回到了列表的开头！
        */
        let tx_fut = async move {
            let vals = vec![String::from("hi"), String::from("from"), String::from("the"), String::from("future")];
            for val in vals {
                tx.send(val).unwrap();
                trpl::sleep(Duration::from_millis(500)).await;
            }
        };
        let rx_fut = async {
            while let Some(value) = rx.recv().await {
                println!("received '{value}'");
            }
        };
        trpl::join(tx_fut, rx_fut).await;
    });
}

fn test33() {
    trpl::run(async {
        let (tx, mut rx) = trpl::channel();

        let tx1 = tx.clone();
        let tx1_fut = async move {
            let vals = vec![String::from("hi"), String::from("from"), String::from("the"), String::from("future")];

            for val in vals {
                tx1.send(val).unwrap();
                trpl::sleep(Duration::from_millis(500)).await;
            }
        };

        let rx_fut = async {
            while let Some(value) = rx.recv().await {
                println!("received '{value}'");
            }
        };

        let tx_fut = async move {
            let vals = vec![String::from("more"), String::from("messages"), String::from("for"), String::from("you")];

            for val in vals {
                tx.send(val).unwrap();
                trpl::sleep(Duration::from_millis(1500)).await;
            }
        };

        trpl::join3(tx1_fut, tx_fut, rx_fut).await;
    })
}
