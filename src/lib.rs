//纯库项目入口文件【例如：序列化库】,本库、子库等不会被打包成二进制文件，主要是提供可复用的代码库

pub mod learn_06_10;
pub mod learn_11_15;
pub mod learn_16_20;
pub mod learn_21_22;
pub mod learn_more;
pub mod test;

pub fn try_some() {
    println!("lib run start: ^_^\n");
    // learn_16_20::learn_20_advanced::learn_20_05::run();
    // learn_more::learn_other_crate::learn_axum::run();
    learn_more::learn_algorithm::learn_01_20::run();
    // test::run();
}
