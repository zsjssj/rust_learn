/*
  使用迭代器处理元素序列
  参考：https://kaisery.github.io/trpl-zh-cn/ch13-02-iterators.html
*/
#![allow(unused_variables)]
#![allow(dead_code)]

pub fn run() {
    test5();
}

//1. 使用 for 循环来迭代集合
fn test1() {
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();
    for val in v1_iter {
        println!("Got: {val}");
    }
}

//2. 直接调用迭代器的 next 方法
fn test2() {
    let v1 = vec![1, 2, 3];
    let mut v1_iter = v1.iter();
    assert_eq!(v1_iter.next(), Some(&1));
    assert_eq!(v1_iter.next(), Some(&2));
    assert_eq!(v1_iter.next(), Some(&3));
    assert_eq!(v1_iter.next(), None);
}

//3. 消费迭代器的方法
fn test3() {
    let v1: Vec<i32> = vec![1, 2, 3];
    let v1_iter: std::slice::Iter<'_, i32> = v1.iter();
    let num: i32 = v1_iter.sum::<i32>();
    println!("sum is: {num}"); //6
}

//4. 适配器方法
fn test4() {
    let v1 = vec![1, 2, 3];
    // 通过 map 方法对每个元素加 1,并收集到一个新的向量中
    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();
    println!("v2 is: {:?}", v2); // [2, 3, 4]
}

//5. 使用闭包作为参数来过滤一个鞋子列表
fn test5() {
    let shoes = vec![
        Shoe { size: 10, style: String::from("sneaker") },
        Shoe { size: 13, style: String::from("sandal") },
        Shoe { size: 10, style: String::from("boot") },
    ];
    let in_my_size = shoes_in_size(shoes, 10);
    println!("shoes in my size: {:?}", in_my_size);
}

#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}

fn shoes_in_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes.into_iter().filter(|s| s.size == shoe_size).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn filters_by_size() {
        let shoes = vec![
            Shoe { size: 10, style: String::from("sneaker") },
            Shoe { size: 13, style: String::from("sandal") },
            Shoe { size: 10, style: String::from("boot") },
        ];
        let in_my_size = shoes_in_size(shoes, 10);
        assert_eq!(in_my_size, vec![Shoe { size: 10, style: String::from("sneaker") }, Shoe { size: 10, style: String::from("boot") },]);
    }
}
