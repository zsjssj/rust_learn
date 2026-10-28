use std::collections::HashMap;

pub fn run() {
    let mut m1 = HashMap::from([("zsj", 26), ("ssje", 25)]);
    test1(&mut m1);
}

// 测试entry以及相关方法
fn test1(m: &mut HashMap<&str, i32>) {
    // m1.entry("gh").and_modify(|x| *x = 26).or_insert(25); //通过and_modify方法来修改
    *m.entry("gh").or_insert(25) += 1; //通过解析地址

    let p1 = m.get("gh");
    match p1 {
        Some(age) => println!("age: {}", age),
        None => println!("插入失败"),
    };
}
