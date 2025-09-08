use std::cell::Cell;

pub fn test() {
    let c = Cell::new("asdf");
    let one = c.get();
    c.set("qwer");
    let two = c.get();
    println!("{},{}", one, two);

    // let s = RefCell::new(String::from("hello, world"));
    // let s1 = s.borrow();
    // let s2 = s.borrow_mut();
    // println!("{},{}", s1, s2);
}
