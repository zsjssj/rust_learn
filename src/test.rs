#![allow(unused)]

pub fn run() {
    let res: Result<i32, &str> = Ok(10);
    let s: Result<String, String> = Ok(String::from("hello"));

    assert_eq!(s.clone().is_ok_and(|x| x.len() >= 5), true); // Ok 里面的值满足 >= 5
    println!("res: {:?}", s);

    // Ok，并且里面的值满足 > 5
    assert_eq!(res.is_ok_and(|x| x > 5), true);
}
