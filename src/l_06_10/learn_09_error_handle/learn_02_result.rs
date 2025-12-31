#![allow(unused)]

use std::fs::File;
use std::io::{self, ErrorKind};

pub fn test() {
    // Result<T, E> 是一个枚举类型，表示一个操作的结果。它有两个变体：Ok(T) 和 Err(E)。
    // enum Result<T, K> {
    //     Ok(T),
    //     Err(K),
    // }
    let a1 = Result::<u32, String>::Err("error".to_string());
    println!("Result: {:?}", a1);
    test04()
    // test02();
}

//match处理
fn test01() {
    let f = File::open("src/learn_error_handle/hello.txt");
    let f1 = match f {
        Ok(file) => file,
        Err(error) => {
            panic!("There is a problem opening the file: {:?}", error);
        }
    };
}

//匹配不同错误【match】
fn test02() {
    let f = File::open("src/learn_error_handle/hello.txt");
    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("src/learn_error_handle/hello.txt") {
                Ok(fc) => {
                    println!("File created successfully: {:?}", fc);
                    fc
                }
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => panic!("Problem opening the file: {:?}", other_error),
        },
    };
    println!("File opened successfully: {:?}", f);
}

//匹配不同错误【if】
fn test03() {
    let f = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });
}

fn test04() {
    let mut guess = String::new();

    loop {
        io::stdin().read_line(&mut guess).expect("Failed to read line");
        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        if guess < 1 || guess > 100 {
            println!("The secret number will be between 1 and 100.");
            continue;
        }

        //     match guess.cmp(&secret_number) {
        // // --snip--
        // }
    }
}
