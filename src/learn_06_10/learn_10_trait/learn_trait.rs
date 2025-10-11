#![allow(unused)]
//共享行为，一种抽象的方式，类似于接口(interfaces)

pub fn test() {
    test007();
}

fn test001() {
    trait Summary {
        fn summarize(&self) -> String;
    }
}

//为类型实现trait、默认实现
fn test002() {
    trait Summary {
        fn summarize(&self) -> String;
        fn summarize_author(&self) -> String {
            String::from("author")
        }
    }

    struct NewsArticle {
        pub headline: String,
        pub location: String,
        pub author: String,
        pub content: String,
    }

    impl Summary for NewsArticle {
        fn summarize(&self) -> String {
            format!("{}, by {} ({})", self.headline, self.author, self.location)
        }
    }

    struct Tweet {
        pub username: String,
        pub content: String,
        pub reply: bool,
        pub retweet: bool,
    }

    impl Summary for Tweet {
        fn summarize(&self) -> String {
            format!("{}: {}", self.username, self.content)
        }
    }

    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };

    let news_article_01 = NewsArticle {
        headline: String::from("book"),
        location: String::from("sichaun"),
        author: String::from("ssje"),
        content: String::from("anything"),
    };

    println!("1 new tweet: {}", tweet.summarize());
    println!("1 new article: {}", news_article_01.summarize_author());
}

//trait作为参数
fn test003() {
    trait Summary {
        fn summarize(&self) -> String;
    }
    struct Tweet {
        pub username: String,
        pub content: String,
        pub reply: bool,
        pub retweet: bool,
    }

    impl Summary for Tweet {
        fn summarize(&self) -> String {
            format!("{}: {}", self.username, self.content)
        }
    }

    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };
    fn notify(item: impl Summary) {
        println!("Breaking news! {}", item.summarize());
    }
    notify(tweet);
}

//trait bound 较长形式语法的语法糖
fn test004() {

    //1.impl Trait方法: pub fn notify(item1: impl Summary, item2: impl Summary) {

    //2.impl Trait方法: pub fn notify<T: Summary>(item1: T, item2: T) { ,强制要求同类型

    //3.通过 + 指定多个 trait bound
    //pub fn notify(item: impl Summary + Display) {
    //impl Trait方法: fn notify<T: Summary + Display>(item: T) {

    //4.通过 where 简化 trait bound
    //fn some_function<T: Display + Clone, U: Clone + Debug>(t: T, u: U) -> i32 {
    /*
        fn some_function<T, U>(t: T, u: U) -> i32
        where T: Display + Clone,
              U: Clone + Debug
    {
         */
}

//返回实现了impl Trait的类型
fn test005() {
    trait Summary {
        fn summarize(&self) -> String;
    }
    struct Tweet {
        pub username: String,
        pub content: String,
        pub reply: bool,
        pub retweet: bool,
    }

    impl Summary for Tweet {
        fn summarize(&self) -> String {
            format!("{}: {}", self.username, self.content)
        }
    }

    /*
    指定了 returns_summarizable 函数返回某个实现了 Summary trait 的类型，但是不确定其具体的类型。
    在这个例子中 returns_summarizable 返回了一个 Tweet，不过调用方并不知情
    只适用于返回单一类型的情况!!!
     */
    fn returns_summarizable() -> impl Summary {
        Tweet {
            username: String::from("horse_ebooks"),
            content: String::from("of course, as you probably already know, people"),
            reply: false,
            retweet: false,
        }
    }
}

fn test006() {
    fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
        let mut largest = list[0];

        for &item in list.iter() {
            if item > largest {
                largest = item;
            }
        }
        largest
    }
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q', 'z'];

    let result = largest(&char_list);
    println!("The largest char is {}", result);
    println!("{}", 'A'.eq_ignore_ascii_case(&'a'))
}

//使用 trait bound 有条件地实现方法
fn test007() {
    use std::fmt::Display;

    struct Pair<T> {
        x: T,
        y: T,
    }

    impl<T> Pair<T> {
        fn new(x: T, y: T) -> Self {
            Self { x, y }
        }
    }

    impl<T: Display + PartialOrd> Pair<T> {
        fn cmp_display(&self) {
            if self.x >= self.y {
                println!("The largest member is x = {}", self.x);
            } else {
                println!("The largest member is y = {}", self.y);
            }
        }
    }
    let pair1 = Pair::new(1, 2);
    let pair2 = Pair { x: 1, y: 2 };
    pair1.cmp_display();

    println!("test007 run end:{}", pair1.x);
}
