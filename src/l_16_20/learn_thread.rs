#![allow(unused)]
use std::clone;
use std::sync::{Arc, Mutex};
use std::thread::{self, JoinHandle};

pub fn try_thread_lock() {
    let v: Arc<Mutex<Vec<i32>>> = Arc::new(Mutex::new(vec![]));
    let mut handles: Vec<JoinHandle<()>> = vec![];
    for i in 0..5 {
        let v: Arc<Mutex<Vec<i32>>> = Arc::clone(&v);
        let handle: JoinHandle<()> = thread::spawn(move || {
            if let Ok(mut vec) = v.lock() {
                vec.push(i);
            } else {
                eprintln!("Failed to acquire lock");
            }
        });
        handles.push(handle);
    }
    for h in handles {
        h.join().unwrap();
    }
    println!("Result: {:?}", *v.lock().unwrap());
}

struct Color(u8, u8, u8, f32);
impl Color {
    fn rgb(&self) -> (u8, u8, u8, f32) {
        (self.0, self.1, self.2, self.3)
    }
}
