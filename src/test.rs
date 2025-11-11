pub fn run() {
    let o1: Option<i32> = Some(2);
    let o2: Option<i32> = o1.and_then(|x| Some(x * 3));
    println!("o1: {:?}", o1);
    println!("o2: {:?}", o2);
}
