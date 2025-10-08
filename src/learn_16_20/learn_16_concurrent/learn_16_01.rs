/*
  使用线程同时运行代码
  参考：https://kaisery.github.io/trpl-zh-cn/ch16-01-threads.html
*/
#![allow(dead_code)]
#![allow(unused_variables)]
use std::thread;
use std::time::Duration;

pub fn run() {
    test3();
}

// 创建一个新线程
fn test1() {
    thread::spawn(|| {
        for i in 1..=10 {
            println!("hi number {i} from the spawned thread!");
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..=5 {
        println!("hi number {i} from the main thread!");
        thread::sleep(Duration::from_millis(1));
    }
}

//使用 join 等待所有线程结束
/*
  thread::spawn 的返回值类型是 JoinHandle<T>。JoinHandle<T> 是一个拥有所有权的值，当对其调用 join 方法时，它会等待其线程结束
  通过调用句柄的 join 会阻塞当前线程直到句柄所代表的线程结束
*/
fn test2() {
    let handle = thread::spawn(|| {
        for i in 1..=10 {
            println!("hi number {i} from the spawned thread!");
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..=5 {
        println!("hi number {i} from the main thread!");
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap(); //会阻塞主线程执行，直到子线程结束
    //主线程等待子线程结束后再继续往下执行
    for i in 100..=105 {
        println!("hi number {i} from the main thread!");
        thread::sleep(Duration::from_millis(1));
    }
}

//将 move 闭包与线程一同使用
fn test3() {
    let v: Vec<i32> = vec![1, 2, 3];
    let handle: thread::JoinHandle<()> = thread::spawn(move || {
        println!("Here's a vector: {v:?}");
    });

    handle.join().unwrap();
}
