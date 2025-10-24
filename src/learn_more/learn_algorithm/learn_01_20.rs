//
pub fn run() {
    let v: Vec<i32> = vec![2, 7, 11, 15];
    let target: i32 = 9;
    learn_01(v, target);
}

//1.两数之和,
pub fn learn_01(v: Vec<i32>, target: i32) {
    //记录运行时间
    let start: std::time::Instant = std::time::Instant::now();
    use std::collections::HashMap;
    let mut map: HashMap<i32, usize> = HashMap::new();
    for (i, &num) in v.iter().enumerate() {
        let complement: i32 = target - num;
        if let Some(&index) = map.get(&complement) {
            println!("Indices: [{}, {}]", index, i);
            // return;
        }
        map.insert(num, i);
    }
    let duration = start.elapsed();
    println!("Time elapsed in learn_01() is: {:?}", duration);
}
