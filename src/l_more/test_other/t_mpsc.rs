#![allow(unused_imports)]
use tokio::sync::mpsc;
use tokio::time::{Duration, sleep};

#[tokio::main(worker_threads = 4)]
pub async fn run() {
    demo_bounded().await;
    // demo_unbounded().await;
    // demo_multiple_producers().await;
    // demo_blocking_send_from_thread().await;
}

/// 1. 有界通道：容量满时 send 会 .await 等待
async fn demo_bounded() {
    println!("=== bounded channel ===");
    let (tx, mut rx) = mpsc::channel::<i32>(2); // 容量 2
    tokio::spawn(async move {
        for i in 1..=5 {
            // 容量满时这里会挂起，让出执行权，而不是阻塞线程
            tx.send(i).await.unwrap();
            println!("sent {i}");
        }
        // tx 在这里 drop，rx.recv() 最终返回 None
    });

    while let Some(v) = rx.recv().await {
        println!("recv {v}");
        sleep(Duration::from_millis(100)).await; // 模拟慢消费者
    }
    println!("channel closed\n");
}

/// 2. 无界通道：send 不是 async，永不因容量等待
async fn demo_unbounded() {
    println!("=== unbounded channel ===");
    let (tx, mut rx) = mpsc::unbounded_channel::<String>();

    tokio::spawn(async move {
        // send 返回 Result，不是 Future，不需要 .await
        tx.send("hello".to_string()).unwrap();
        tx.send("world".to_string()).unwrap();
    });

    while let Some(msg) = rx.recv().await {
        println!("recv {msg}");
    }
    println!();
}

/// 3. 多生产者：clone sender 即可
async fn demo_multiple_producers() {
    println!("=== multiple producers ===");
    let (tx, mut rx) = mpsc::channel::<&'static str>(8);

    for name in ["A", "B", "C"] {
        let tx = tx.clone();
        tokio::spawn(async move {
            tx.send(name).await.unwrap();
        });
    }
    drop(tx); // 必须 drop 原始 tx，否则 rx 永远等不到 None

    let mut count = 0;
    while let Some(v) = rx.recv().await {
        println!("recv from {v}");
        count += 1;
    }
    println!("total {count} messages\n");
}

/// 4. 从同步线程向异步任务发消息：blocking_send
async fn demo_blocking_send_from_thread() {
    println!("=== blocking_send from std thread ===");
    let (tx, mut rx) = mpsc::channel::<i32>(4);

    // 普通 OS 线程（非 tokio 任务）
    std::thread::spawn(move || {
        for i in 0..3 {
            // 在同步上下文中用 blocking_send，它会阻塞当前线程直到有空间
            // 注意：不能在 async 任务里调用 blocking_send，会 panic
            tx.blocking_send(i).unwrap();
        }
        // tx drop 后，rx 收到 None
    });

    while let Some(v) = rx.recv().await {
        println!("recv {v}");
    }
    println!();
}
