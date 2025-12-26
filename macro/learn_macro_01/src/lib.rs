//声明式宏
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_macros)]

use rand::Rng;

///创建一个函数，函数名由参数指定，函数返回Result<Option<i32>, String>
#[macro_export]
macro_rules! create_function {
    ($func_name:ident) => {
        fn $func_name() -> Result<Option<i32>, String> {
            println!("You called {:?}()", stringify!($func_name));
            let mut rng = rand::rng();
            let n = rng.random_bool(0.5);
            if n {
                Err("An error occurred".to_string())
            } else {
                Ok(Some(1))
            }
        }
    };
}
