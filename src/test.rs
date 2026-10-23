#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

use anyhow::Result;
use dotenvy;
use std::{env, marker::PhantomPinned, pin::Pin};

#[tokio::main]
pub async fn run() {
    test_1().unwrap();
}

struct SelfRef {
    data: String,
    ptr: *const String,  // 指向自己的 data
    _pin: PhantomPinned, // 关键：让类型 !Unpin
}

impl SelfRef {
    fn new(data: String) -> Self {
        SelfRef {
            data,
            ptr: std::ptr::null(),
            _pin: PhantomPinned,
        }
    }
    fn init(self: Pin<&mut Self>) {
        // 安全地拿到内部可变引用（unsafe，因为要设置自引用）
        let this = unsafe { self.get_unchecked_mut() };
        this.ptr = &this.data;
    }
    fn data(&self) -> &String {
        &self.data
    }
    fn ptr_data(&self) -> &String {
        unsafe { &*self.ptr }
    }
}
//测试dotenvy的用法
fn test_1() -> Result<()> {
    let mut s = SelfRef::new("hello".into());
    s.ptr = &s.data; // 自引用
    let s2 = s; // ⚠️ move：s 的地址变了，但 s2.ptr 仍指向旧地址！
    // s2.ptr_data() 现在是未定义行为

    let mut s = Box::pin(SelfRef::new("hello".into()));
    s.as_mut().init(); // 在堆上固定地址后建立自引用
    // 下面这行会编译失败：
    // let s2 = s;    // ❌ 不能 move 出 Pin<Box<T>>
    // let s3 = *s;   // ❌ cannot move out of `Pin<Box<SelfRef>>`

    Ok(())
}
