//!   使用消息传递在线程间传送数据
/*
  参考：https://kaisery.github.io/trpl-zh-cn/ch16-02-message-passing.html
*/
#![allow(dead_code)]
#![allow(unused_variables)]
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

pub fn run() {
    // test1();
    // test2();
    test3();
}

fn test1() {
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap(); //会拥有 val 的所有权，并将其发送到另一个线程
        // println!("val is: {val}"); //编译错误，因为 val 的所有权已经被转移
    });
    /*
          信道的接收端有两个有用的方法：recv 和 try_recv。
          这里，使用了 recv，它是 receive 的缩写，这会阻塞主线程执行直到从信道中接收一个值。
      一旦发送了一个值，recv 会在一个 Result<T, E> 中返回它。当信道发送端关闭，recv 会返回一个错误表明不会再有新的值到来了。
          try_recv 不会阻塞，相反它立刻返回一个 Result<T, E>：Ok 值包含可用的信息，而 Err 值代表此时没有任何消息。
      如果线程在等待消息过程中还有其他工作时使用 try_recv 很有用：可以编写一个循环来频繁调用 try_recv，在有可用消息时进行处理，其余时候则处理一会其他工作直到再次检查。
    */
    let received = rx.recv().unwrap();
    println!("Got: {received}");
}

///发送多个值并观察接收端的等待
fn test2() {
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        let vals = vec![String::from("hi"), String::from("from"), String::from("the"), String::from("thread")];
        for val in vals.into_iter() {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });
    for received in rx {
        println!("Got: {received}");
    }
}

///通过克隆发送端来创建多个生产者
fn test3() {
    let (tx, rx) = mpsc::channel();
    let tx1 = tx.clone();
    thread::spawn(move || {
        let vals = vec![String::from("hi"), String::from("from"), String::from("the"), String::from("thread")];
        for v in vals.into_iter() {
            tx1.send(v).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });
    thread::spawn(move || {
        let vals = vec![String::from("more"), String::from("messages"), String::from("for"), String::from("you")];
        for val in vals.into_iter() {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });
    for received in rx {
        println!("Got: {received}");
    }
}
