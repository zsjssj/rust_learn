pub fn test() {
    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s2 is {}", s2);
    test_01();
    test_02();
}

fn test_01() {
    let s1 = String::from("tic");
    let mut s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = s1 + "-" + &s2 + "-" + &s3; //s1被移动了,不能再使用,会失去s1的所有权
    s2.push_str("!!!");
    println!("s is {}", s);
    println!("s2 is {}", s2);
}

fn test_02() {
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = format!("{}-{}-{}", s1, s2, s3); //s1没有被移动,可以继续使用
    println!();
    println!("s is {}", s);
    println!("s2 is {}", s2);
}
