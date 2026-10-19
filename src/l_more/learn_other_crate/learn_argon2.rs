/// 体会argon2加密【cpu密集计算】
use argon2::{Argon2, PasswordHasher};

pub fn hash_password(password: &str) -> Result<String, String> {
    // let algorithm = argon2::Algorithm::Argon2d;
    // let version = argon2::Version::V0x10;
    // let params = argon2::Params::new(10 * 1024 , 10, 10, Some(10)).unwrap(); //内存成本、迭代次数、并行度
    // Argon2::new(algorithm, version, params);
    Argon2::default()
        .hash_password(password.as_bytes())
        .map(|hash| hash.to_string())
        .map_err(|_| "密码哈希失败".to_string())
}

pub fn run() {
    let t1 = std::time::Instant::now();
    let password = "123456".to_string();
    let a = password.as_bytes();
    let p_h = hash_password(&password);
    let d = t1.elapsed().as_secs_f64() * 1000.0; //平均用时300ms
    println!("运行时间：{} ms", d)
}
