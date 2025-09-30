/*
  使用 Drop Trait 运行清理代码
  参考：https://kaisery.github.io/trpl-zh-cn/ch15-03-drop.html
*/
#![allow(dead_code)]
#![allow(unused_variables)]

pub fn run() {
    test1();
}

//1. 使用 Drop Trait 运行清理代码，
fn test1() {
    let c1: CustomSmartPointer = CustomSmartPointer { data: String::from("my stuff") };
    let c2: CustomSmartPointer = CustomSmartPointer { data: String::from("other stuff") };
    println!("CustomSmartPointers created.");
    /*
     手动调用 drop 方法. 这会导致编译错误，因为 drop 方法是由 Drop trait 提供的，不能手动调用
     应该使用的是由标准库提供的 std::mem::drop 函数
    */
    // c1.drop();
    drop(c1); //std::mem::drop 位于prelude中，可以直接使用
    println!("CustomSmartPointer dropped before the end of main.");
}
struct CustomSmartPointer {
    data: String,
}
impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}
