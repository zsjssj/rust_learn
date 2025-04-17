// panic!() 是一个宏,用于在运行时引发 panic
#![allow(unused)]

pub fn test() {
    test_02();
}

fn test_01() {
    panic!("crash and burn");
}
fn test_02() {
    let v = vec![1, 2, 3];
    v[99];
}
