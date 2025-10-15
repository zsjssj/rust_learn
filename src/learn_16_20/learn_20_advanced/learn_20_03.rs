//!高级类型
//参考：https://kaisery.github.io/trpl-zh-cn/ch20-03-advanced-types.html
#![allow(dead_code)]
#![allow(unused_variables)]

pub fn run() {
    test1();
    // test2();
}

//使用 newtype 模式实现类型安全和抽象
fn test1() {
    //1.类型别名
    type Kilometers = i32;
    let x: i32 = 5;
    let y: Kilometers = 5;
    println!("x + y = {}", x + y);

    //2.引入类型别名 Thunk 来减少重复
    type Thunk = Box<dyn Fn() + Send + 'static>;
    let f: Thunk = Box::new(|| println!("hi"));
    fn takes_long_type(f: Thunk) {
        // --snip--
    }
    fn returns_long_type() -> Thunk {
        // --snip--
        Box::new(|| println!("hi"))
    }

    //3.完全限定的别名
    type Result<T> = std::result::Result<T, std::io::Error>;
    pub trait Write {
        fn write(&mut self, buf: &[u8]) -> Result<usize>;
        fn flush(&mut self) -> Result<()>;

        fn write_all(&mut self, buf: &[u8]) -> Result<()>;
        fn write_fmt(&mut self, fmt: std::fmt::Arguments) -> Result<()>;
    }
}

//从不返回的 never type
fn test2() {
    /*
    【 ! 】:特殊类型，在类型理论术语中被称为 empty type，因为它没有值。
    我们更倾向于称之为 never type
    */
    /*
     这段代码可以读作 “函数 bar 从不返回”，而从不返回的函数被称为 发散函数（diverging functions）。
     不能创建 ! 类型的值，所以 bar 也不可能返回值。
    */
    // fn bar() -> ! {
    //     // --snip-- //报错
    // }
}

//动态大小类型和 Sized trait
fn test3() {
    /*
      // 这段代码无法编译，因为 str 是一种 DST(动态大小类型)，它的大小在编译时未知
      let s1: str = "Hello there!";
      let s2: str = "How's it going?";
    */

    fn generic<T>(t: T) {
        // --snip--
    }
    /*
     实际上，这会被当作我们写了如下内容来处理：
      fn generic<T: Sized>(t: T) {
          // --snip--
      }
    */

    //默认情况下，泛型函数只能作用于在编译时大小已知的类型。然而，你可以使用如下特殊语法来放宽这一限制：
    fn generic1<T: ?Sized>(t: &T) {
        // --snip--
    }
}
