/*
  使用 Deref Trait 将智能指针当作常规引用处理
  参考：https://kaisery.github.io/trpl-zh-cn/ch15-02-deref.html
*/
#![allow(dead_code)]
#![allow(unused_variables)]

pub fn run() {
    test3();
}

//1. 使用 Deref Trait 解引用
fn test1() {
    let x: i32 = 5;
    let y: &i32 = &x;
    assert_eq!(5, x);
    assert_eq!(5, *y); //手动解引用，判断y指向的值是否等于5

    let x: i32 = 9;
    let y: Box<i32> = Box::new(x); //像引用一样使用 Box<T>
    assert_eq!(9, x);
    assert_eq!(9, *y); //手动解引用，判断y指向的值是否等于9
}

//2. 自定义智能指针，重点是实现 Deref Trait 。未实现将数据放入堆内存
use std::ops::Deref;
struct MyBox<T>(T);
impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}
impl<T> Deref for MyBox<T> {
    type Target = T; //关联类型
    fn deref(&self) -> &Self::Target {
        &self.0 //返回指向值的引用
    }
}
fn test2() {
    let x: i32 = 5;
    let y: MyBox<i32> = MyBox::new(x);
    assert_eq!(5, x);
    assert_eq!(5, *y); //手动解引用，判断y指向的值是否等于5
}

//3. 函数和方法的隐式 Deref 强制转换
/*
  这里使用 &m 调用 hello 函数，其为 MyBox<String> 值的引用。
  因为示例 15-10 中在 MyBox<T> 上实现了 Deref trait，
  Rust 可以通过 deref 调用将 &MyBox<String> 变为 &String。
  标准库中提供了 String 上的 Deref 实现，其会返回字符串 slice，
  这可以在 Deref 的 API 文档中看到。Rust 再次调用 deref 将 &String 变为 &str，这就符合 hello 函数的定义了
*/
fn test3() {
    let m = MyBox::new(String::from("Rust"));
    hello(&m); // &MyBox<String> -> String -> &str----自动解引用以达成转换
    hello(&(*m)[..]); // &MyBox<String> -> String -> &str----手动解引用以达成转换
}
fn hello(name: &str) {
    println!("Hello, {name}!");
}

//4. Deref 强制转换如何与可变性交互
/*
Rust 在发现类型和 trait 实现满足三种情况时会进行 Deref 强制转换：
  1. 当 T: Deref<Target=U> 时从 &T 到 &U。
  2. 当 T: DerefMut<Target=U> 时从 &mut T 到 &mut U。
  3. 当 T: Deref<Target=U> 时从 &mut T 到 &U。
*/
fn test4() {
    let mut x: i32 = 5;
    let y: MyBox<i32> = MyBox::new(x);
    // *y += 10; //error,无法编译，因为y是不可变的
    let z: &mut i32 = &mut x; //获取x的可变引用
    *z += 10; //通过z修改x的值
    assert_eq!(15, x);
}
