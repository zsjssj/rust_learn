//! 不安全 Rust
//参考：https://kaisery.github.io/trpl-zh-cn/ch20-01-unsafe-rust.html
//不安全 Rust 允许你绕过 Rust 的某些安全检查。你可以在不安全代码块中执行以下操作：
//解引用裸指针
//调用不安全函数或方法
//访问或修改可变静态变量
//实现不安全 trait
//使用 union
//不安全代码块使用 unsafe 关键字标记
//不安全代码块中可以包含安全代码
//不安全代码块可以嵌套

#![allow(dead_code)]
#![allow(unused_variables)]

pub fn run() {
    // test1();
    // test2();
    // test3();
    test4();
}
//解引用裸指针
fn test1() {
    let mut num = 5;
    //裸指针借用操作符（raw borrow operators）
    //&raw const num 会创建一个 *const i32 的不可变裸指针
    let r1 = &raw const num;
    let r2 = &raw mut num;
    //解引用裸指针必须在不安全代码块中进行
    unsafe {
        println!("r1 is: {}", *r1);
        println!("r2 is: {}", *r2);
    }

    //将创建一个有效性无法确定的裸指针，
    let address = 0x012345usize;
    let r = address as *const i32;
}

//调用不安全函数或方法
unsafe fn dangerous() {}
fn test2() {
    unsafe {
        dangerous();
    }

    let mut v: Vec<i32> = vec![1, 2, 3, 4, 5, 6];
    let r: &mut [i32] = &mut v[..];
    let (a, b) = r.split_at_mut(3);
    assert_eq!(a, &mut [1, 2, 3]);
    assert_eq!(b, &mut [4, 5, 6]);
    println!("a: {:?}, b: {:?}", a, b);

    let (a1, b1) = split_at_mut(r, 3);
    println!("a1: {:?}, b1: {:?}", a1, b1);
    assert_eq!(a1, &mut [1, 2, 3]);
    assert_eq!(b1, &mut [4, 5, 6]);

    //创建一个有效性无法确定的裸指针,运行时候可能崩溃
    let address = 0x01234usize;
    let r = address as *mut i32;
    let values: &[i32] = unsafe { slice::from_raw_parts_mut(r, 10000) };
}
//将 split_at_mut 实现为函数
use std::slice;
fn split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    // let len = values.len();
    // assert!(mid <= len);
    // (&mut values[..mid], &mut values[mid..]) //错误：不能同时借用 values 的两个可变引用

    let len = values.len();
    let ptr = values.as_mut_ptr();
    println!("ptr is: {:?}", ptr);
    assert!(mid <= len);
    unsafe { (slice::from_raw_parts_mut(ptr, mid), slice::from_raw_parts_mut(ptr.add(mid), len - mid)) }
}

//使用 extern 函数调用外部代码
fn test3() {
    /*
    unsafe extern 中声明的任何项都隐式地是 unsafe 的。然而，一些 FFI 函数可以安全地调用。
    C 标准库中的 abs 函数没有任何内存安全方面的考量并且我们知道它可以使用任何 i32 调用。
    在类似这样的例子中，我们可以使用 safe 关键字来表明这个特定的函数即便是在 unsafe extern 块中也是可以安全调用的。
    一旦我们做出这个修改，调用它不再需要 unsafe 块。
     */
    //1.
    // unsafe {
    //     println!("Absolute value of -3 according to C: {}", abs(-3));
    // }
    println!("Absolute value of -3 according to C: {}", abs(-3));
}
//1.
// unsafe extern "C" {
//     fn abs(input: i32) -> i32;
// }
unsafe extern "C" {
    safe fn abs(input: i32) -> i32;
}

//从其它语言调用 Rust 函数
#[unsafe(no_mangle)] //告诉 Rust 不要改变这个函数的名字
pub extern "C" fn call_from_c() {
    println!("Just called a Rust function from C!");
}

//访问或修改可变静态变量
fn test4() {
    /*
      任何读写 COUNTER 的代码都必须位于 unsafe 块中。
      静态变量在程序的整个生命周期内都存在，因此它们有 'static 生命周期。
      可变静态变量类似于全局变量，因此在任何时候只能有一个线程访问它们。
      这就是为什么访问或修改可变静态变量是不安全的：Rust 无法保证同时只有一个线程在访问它们。
      你必须手动确保同一时间只有一个线程在访问可变静态变量。
      下面的例子中，add_to_count 函数增加 COUNTER 的值
    */
    unsafe {
        // SAFETY: 它只在 `main` 这一个线程被调用。
        add_to_count(3);
        println!("COUNTER: {}", *(&raw const COUNTER));
    }
}
static mut COUNTER: u32 = 0;
/// SAFETY: 同时在多个线程调用这个方法是未定义的行为，所以你*必须*保证同一时间只
/// 有一个线程在调用它。
unsafe fn add_to_count(inc: u32) {
    unsafe {
        COUNTER += inc;
    }
}

//实现不安全 trait
fn test5() {}
/*
  当 trait 中至少有一个方法中包含编译器无法验证的不变式（invariant）时该 trait 就是不安全的。
*/
unsafe trait Foo {
    // 方法在这里
}
unsafe impl Foo for i32 {
    // 方法实现在这里
}
