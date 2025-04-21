#![allow(unused)]

pub fn test() {
    let a11 = Coin::Quarter;
    let a1 = value_in_cents(a11);
    let a2 = value_in_cents(Coin::Penny);
    println!("a1: {}, a2: {}", a1, a2);
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}
