#![allow(unused)]
// use std::collections::HashMap;
use std::pin::Pin;

pub fn run() {
    test1();
}
fn test1() {
    let dog1 = Dog { name: String::from("Rex") };
    let dog2 = Dog { name: String::from("Fluffy") };
    running1(&dog1);
    running2(&dog2);
    let p1: Person = Person {
        name: String::from("Alice"),
        age: 30,
        pet: vec![Box::new(dog1), Box::new(dog2)],
    };
    running1(&p1);
    for a in p1.pet.iter() {
        running2(a.as_ref());
    }
}
trait Animal {
    fn run(&self) {
        println!("run fast");
    }
}
fn running1<T: Animal>(animal: &T) {
    print!("running1:  ");
    animal.run();
}
fn running2(animal: &dyn Animal) {
    print!("running2:  ");
    animal.run();
}

struct Person {
    name: String,
    age: u8,
    pet: Vec<Box<dyn Animal>>,
}

impl Animal for Person {}
struct Dog {
    name: String,
}
impl Animal for Dog {}
