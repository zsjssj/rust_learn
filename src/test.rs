#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
use rand::Rng;

pub fn run() {
    // let res = test1().unwrap_or("aaaaaaaaa".to_string());
    // println!("run res is {}", res);
    foo();
}

//you几率返回Err或者Ok
pub fn maybe_err() -> Result<Option<i32>, String> {
    let mut rng = rand::rng();
    let n = rng.random_bool(0.5);
    if n {
        Err("An error occurred".to_string())
    } else {
        Ok(Some(1))
    }
}
fn test1() -> Result<String, String> {
    let res = maybe_err()
        .map_err(|e| format!("获取云端重建地址失败: {}", e))?
        .ok_or(" 为空")?;
    println!("test1 res is {res}");
    Ok("success".to_string())
}

//声明式宏
macro_rules! create_function {
    ($func_name:ident) => {
        fn $func_name() {
            println!("You called {:?}()", stringify!($func_name));
        }
    };
}
create_function!(foo);

//
