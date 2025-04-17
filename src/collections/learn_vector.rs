fn tset1(value: &Vec<u8>) {
    println!("v1[0]: {:?}", value);
}

pub fn test() {
    let mut v1: Vec<u8> = Vec::new();
    let mut v2 = vec![1, 2, 3];
    v1.push(1);
    v2.push(4);
    println!("v1: {:?}", v1);
    println!("v2: {:?}", v2);
    {
        tset1(&v1);
    }

    match v1.first() {
        Some(x) => println!("v1[0]: {}", x),
        None => println!("v1[0] is None"),
    }

    match v2.get(1) {
        Some(x) => println!("v2[1]: {}", x),
        None => println!("v2[1] is None"),
    }

    match v1.get(2) {
        Some(x) => println!("v1[2]: {}", x),
        None => println!("v1[2] is None"),
    }

    let mut v: Vec<i32> = vec![100, 32, 57];

    {
        let a1 = v.get(1);
        println!("a1: {:?}", a1);
    }
    v.push(121);

    for i in &mut v {
        *i += 50; // 这里的 *i 是可变引用,修改值
    }
    for i in &v {
        println!("新的元素：{}", i);
    }
}
