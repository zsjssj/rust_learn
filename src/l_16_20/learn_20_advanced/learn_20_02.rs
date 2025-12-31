//!高级 trait
//参考：https://kaisery.github.io/trpl-zh-cn/ch20-02-advanced-traits.html

#![allow(dead_code)]
#![allow(unused_variables)]
pub fn run() {
    // test1();
    // test2();
    // test21();
    // test3();
    // test4();
    test5();
}

//关联类型
fn test1() {}

//定义 trait 时使用占位符类型，而无需预先知道这些类型的具体内容，直到实现该 trait 时再进行指定
pub trait Iterator {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
}
struct Counter;
impl Iterator for Counter {
    type Item = u32;
    fn next(&mut self) -> Option<Self::Item> {
        Some(0)
    }
}

//默认泛型类型参数和运算符重载
use std::ops::Add;
fn test2() {
    #[derive(Debug, Copy, Clone, PartialEq)]
    struct Point {
        x: i32,
        y: i32,
    }
    impl Add for Point {
        type Output = Point;
        fn add(self, other: Point) -> Point {
            Point { x: self.x + other.x, y: self.y + other.y }
        }
    }
    /*
    默认泛型类型位于 Add trait 中
    Rhs=Self：这个语法叫做 默认类型参数（default type parameters）
    Rhs 是一个泛型类型参数（“right-hand side” 的缩写），它用于定义 add 方法中的 rhs 参数。
    如果实现 Add trait 时不指定 Rhs 的具体类型，Rhs 的类型将默认为 Self，即正在实现 Add 的类型
    trait Add<Rhs=Self> {
        type Output;
        fn add(self, rhs: Rhs) -> Self::Output;
    }
    */
    assert_eq!(Point { x: 1, y: 0 } + Point { x: 2, y: 3 }, Point { x: 3, y: 3 });
    println!("Point add test success!  {:?}", Point { x: 1, y: 0 } + Point { x: 2, y: 3 });
}
//将现有类型简单封装进另一个结构体的方式被称为 newtype 模式
fn test21() {
    struct Millimeters(f32);
    struct Meters(f32);
    impl Add<Meters> for Millimeters {
        type Output = Millimeters;
        fn add(self, other: Meters) -> Millimeters {
            Millimeters(self.0 + (other.0 * 1000.0))
        }
    }
    impl Add<Millimeters> for Meters {
        type Output = Meters;
        fn add(self, other: Millimeters) -> Meters {
            Meters(self.0 + (other.0 / 1000.0))
        }
    }
    let a: Millimeters = Millimeters(500.0);
    let b: Meters = Meters(2.0);
    let c: Millimeters = a + b;
    println!("c is: {} mm", c.0);
    let x: Meters = Meters(5.0);
    let y: Millimeters = Millimeters(1500.0);
    let z: Meters = x + y;
    println!("z is: {} m", z.0);
}

//在同名方法之间消歧义
fn test3() {
    trait Pilot {
        fn fly(&self);
    }
    trait Wizard {
        fn fly(&self);
    }
    struct Human;
    impl Pilot for Human {
        fn fly(&self) {
            println!("This is your captain speaking.");
        }
    }
    impl Wizard for Human {
        fn fly(&self) {
            println!("Up!");
        }
    }
    impl Human {
        fn fly(&self) {
            println!("*waving arms furiously*");
        }
    }
    let person: Human = Human;
    person.fly(); //直接调用 Human 的 fly 方法

    /*
     因为 fly 方法获取一个 self 参数，如果有两个类型都实现了同一 trait，Rust 可以根据 self 的类型计算出应该使用哪一个 trait 实现
    */
    Pilot::fly(&person); //通过显式指定 trait 来调用 fly 方法
    Wizard::fly(&person); //通过显式指定 trait 来调用 fly 方法

    //关联函数中非方法的函数不带有 self 参数
    trait Animal {
        fn baby_name() -> String;
    }
    struct Dog;
    impl Dog {
        fn baby_name() -> String {
            String::from("Spot")
        }
    }
    impl Animal for Dog {
        fn baby_name() -> String {
            String::from("puppy")
        }
    }
    /*
      因为 Animal::baby_name 没有 self 参数，而且可能有其他类型实现了 Animal trait，Rust 无法确定我们想调用哪一个 Animal::baby_name 的实现
    */
    // println!("A baby dog is called a {}", Animal::baby_name()); //报错！！！
    println!("A baby dog is called a {}", Dog::baby_name());
    //【完全限定语法】！！！
    println!("A baby dog is called a {}", <Dog as Animal>::baby_name());
}

//使用超 trait
/*
  需要编写一个依赖另一个 trait 的 trait 定义：对于一个实现了第一个 trait 的类型，你希望要求这个类型也实现了第二个 trait。
  如此就可使 trait 定义使用第二个 trait 的关联项。这个所需的 trait 是我们实现的 trait 的 超（父）trait（supertrait）。
*/
fn test4() {
    use std::fmt;
    trait OutlinePrint: fmt::Display {
        fn outline_print(&self) {
            let output: String = self.to_string();
            let len: usize = output.len();
            println!("{}", "*".repeat(len + 4));
            println!("*{}*", " ".repeat(len + 2));
            println!("* {output} *");
            println!("*{}*", " ".repeat(len + 2));
            println!("{}", "*".repeat(len + 4));
        }
    }
    struct Point {
        x: i32,
        y: i32,
    }
    impl OutlinePrint for Point {}
    impl fmt::Display for Point {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "({}, {})", self.x, self.y)
        }
    }
    let p: Point = Point { x: 1, y: 3 };
    p.outline_print();
    println!("{}", p);
}

//使用 newtype 模式在外部类型上实现外部 trait
fn test5() {
    use std::fmt;
    struct Wrapper(Vec<String>);
    impl fmt::Display for Wrapper {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "[{}]", self.0.join(", "))
        }
    }
    let w = Wrapper(vec![String::from("hello"), String::from("world")]);
    println!("w = {w}");
}
