#![allow(unused)]
use rand::Rng;

pub fn test() {
    //便利方法：每次使用重复获取随机数种子rng，性能略差。
    //1. 取对应类型的随机数
    for i in 0..5 {
        let x1 = rand::random::<u8>();
        let x2 = rand::random::<i8>();
        println!("x1:{x1}  x2:{x2}");
    }
    //2. 取指定概率的布尔值【0，1】
    for i in 0..5 {
        //50%概率为true, 可能有浮点精度误差（比如 1/3 不能精确表示）
        let b1 = rand::random_bool(0.5);
        println!("b1:{b1}");
    }
    //3. 取指定概率的布尔值【0，1】
    for i in 0..5 {
        let b2 = rand::random_ratio(1, 3); //1/3概率为true,适合概率是一个精确分数
        println!("b2:{b2}");
    }
    //4. 取指定范围的随机数
    for i in 0..5 {
        let f1 = rand::random_range(10.0..=20.0);
        println!("f1:{f1}");
    }
    //5. 生成无限个随机 u8
    let mut iter = rand::random_iter::<u8>();
    //取前5个
    for x in iter.take(5) {
        println!("{}", x);
    }
}

pub fn test1() {
    //底层实现
    //1. 生成随机数种子
    let mut rng: rand::prelude::ThreadRng = rand::rng();
    let xx1 = rng.random::<u8>();
    let ff1 = rng.random_bool(0.5);
    let ff2 = rng.random_ratio(1, 3);
    let bb1 = rng.random_range(100..=200);
    println!("xx1:{xx1}  ff1:{ff1}  ff2:{ff2}  bb1:{bb1}");
    //2. 生成无限个随机 u8
    let mut iter2 = rng.random_iter::<u8>();
    for x in iter2.take(5) {
        println!("{}", x);
    }
}
