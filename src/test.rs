#![allow(unused)]

pub fn run() {
    // test1();
    // test2();
    let s = String::from("hello");
    let char = s.chars().nth(1).unwrap();
    println!("{}", char);
}

fn test1() {
    use std::sync::{Arc, Mutex};
    use std::thread;
    let data = Arc::new(Mutex::new(0));
    let threads: Vec<_> = (0..5)
        .map(|_| {
            let data = data.clone();
            thread::spawn(move || {
                let mut num = data.lock().unwrap(); // 阻塞线程直到获得锁
                *num += 1;
            })
        })
        .collect();
    for t in threads {
        t.join().unwrap();
    }
    println!("Result: {}", *data.lock().unwrap());
}

#[tokio::main]
async fn test2() {
    use std::sync::Arc;
    use tokio::sync::Mutex;
    use tokio::task;

    let data = Arc::new(Mutex::new(0));
    let mut handles = vec![];
    for _ in 0..5 {
        let data = data.clone();
        let handle = task::spawn(async move {
            let mut num = data.lock().await; // 挂起任务，不阻塞线程
            *num += 1;
        });
        handles.push(handle);
    }
    for handle in handles.into_iter() {
        handle.await.unwrap();
    }

    println!("Result: {}", *data.lock().await);
}
