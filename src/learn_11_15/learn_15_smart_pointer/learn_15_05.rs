/*
  RefCell<T> 和内部可变性模式
  参考：https://kaisery.github.io/trpl-zh-cn/ch15-05-interior-mutability.html
*/
#![allow(dead_code)]
#![allow(unused_variables)]

pub fn run() {
    test2();
}
fn test1() {}
pub trait Messenger {
    fn send(&self, msg: &str);
}
pub struct LimitTracker<'a, T: Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize,
}
impl<'a, T> LimitTracker<'a, T>
where
    T: Messenger,
{
    pub fn new(messenger: &'a T, max: usize) -> LimitTracker<'a, T> {
        LimitTracker { messenger, value: 0, max }
    }
    pub fn set_value(&mut self, value: usize) {
        self.value = value;
        let percentage_of_max = self.value as f64 / self.max as f64;
        if percentage_of_max >= 1.0 {
            self.messenger.send("Error: You are over your quota!");
        } else if percentage_of_max >= 0.9 {
            self.messenger.send("Urgent warning: You've used up over 90% of your quota!");
        } else if percentage_of_max >= 0.75 {
            self.messenger.send("Warning: You've used up over 75% of your quota!");
        }
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     struct MockMessenger {
//         sent_messages: Vec<String>,
//     }
//     impl MockMessenger {
//         fn new() -> MockMessenger {
//             MockMessenger { sent_messages: vec![] }
//         }
//     }
//     impl Messenger for MockMessenger {
//         fn send(&self, message: &str) {
//             self.sent_messages.push(String::from(message));  //报错
//         }
//     }

//     #[test]
//     fn it_sends_an_over_75_percent_warning_message() {
//         let mock_messenger: MockMessenger = MockMessenger::new();
//         let mut limit_tracker: LimitTracker<'_, MockMessenger> = LimitTracker::new(&mock_messenger, 100);
//         limit_tracker.set_value(80);
//         assert_eq!(mock_messenger.sent_messages.len(), 1);
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    struct MockMessenger {
        sent_messages: RefCell<Vec<String>>,
    }
    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger { sent_messages: RefCell::new(vec![]) }
        }
    }
    impl Messenger for MockMessenger {
        fn send(&self, message: &str) {
            self.sent_messages.borrow_mut().push(String::from(message));
        }
    }
    #[test]
    fn it_sends_an_over_75_percent_warning_message() {
        let mock_messenger: MockMessenger = MockMessenger::new();
        let mut limit_tracker: LimitTracker<'_, MockMessenger> = LimitTracker::new(&mock_messenger, 100);
        limit_tracker.set_value(80);
        assert_eq!(mock_messenger.sent_messages.borrow().len(), 1);
    }
}

use std::cell::RefCell;
use std::rc::Rc;
fn test2() {
    use List::{Cons, Nil};
    let value = Rc::new(RefCell::new(5));
    let a = Rc::new(Cons(value.clone(), Rc::new(Nil)));

    let b = Cons(Rc::new(RefCell::new(3)), a.clone());
    let c = Cons(Rc::new(RefCell::new(4)), a.clone());
    *value.borrow_mut() += 10;

    println!("a after = {a:?}");
    println!("b after = {b:?}");
    println!("c after = {c:?}");
}
#[derive(Debug)]
enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}
