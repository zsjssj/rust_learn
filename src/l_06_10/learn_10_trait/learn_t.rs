#![allow(unused)]

//泛型 T
pub fn test() {
    test003();
}

fn test001() {
    struct Point<T, U> {
        x: T,
        y: U,
    }

    let both_integer = Point { x: 5, y: 10 };
    let both_float = Point { x: 1.0, y: 4.0 };
    let integer_and_float = Point { x: 5, y: 4.0 };
}

fn test002() {
    struct Point<T> {
        x: T,
        y: T,
    }

    impl<T> Point<T> {
        fn x(&self) -> &T {
            &self.x
        }
    }

    let p = Point { x: 5, y: 10 };
    println!("p.x = {}", p.x());
}

fn test003() {
    enum Option<T> {
        Some(T),
        None,
    };
    impl<T> Option<T> {
        fn is_some(&self) -> bool {
            match self {
                Option::Some(_) => true,
                Option::None => false,
            }
        }
    }
    let x = Option::Some(5);

    match x {
        Option::Some(v) => println!("x = {}", v),
        Option::None => println!("x = None"),
    }
    // let y = Option::None;
    println!("x.is_some() = {}", x.is_some());
}

fn test004() {
    struct Point<T, U> {
        x: T,
        y: U,
    }

    impl<T, U> Point<T, U> {
        fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
            Point { x: self.x, y: other.y }
        }
    }

    fn main() {
        let p1 = Point { x: 5, y: 10.4 };
        let p2 = Point { x: "Hello", y: 'c' };

        let p3 = p1.mixup(p2);

        println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
    }
}
