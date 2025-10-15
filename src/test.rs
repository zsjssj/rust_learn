#![allow(unused)]
// use std::collections::HashMap;
use std::pin::Pin;

pub fn run() {
    test1();
}
fn test1() {
    trait Any {
        fn type_id(&self) -> u32 {
            1
        }
    }
    trait Animal: Any {
        fn speak(&self) {
            let num = self.type_id();
            println!("Animal speak, type_id is: {}", num);
        }
    }
    struct Dog;
    impl Any for Dog {}
    impl Animal for Dog {}
    let d = Dog;
    d.speak();
}
