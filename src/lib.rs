//纯库项目入口文件【例如：序列化库】,本库、子库等不会被打包成二进制文件，主要是提供可复用的代码库

pub mod l_06_10;
pub mod l_11_15;
pub mod l_16_20;
pub mod l_21_22;
pub mod l_more;

pub mod test;

pub fn try_some() {
    println!("lib run start: ^_^");

    // learn_16_20::learn_20_advanced::learn_20_05::run();
    // learn_more::learn_other_crate::learn_axum::run();
    // learn_more::learn_algorithm::run();
    // learn_more::learn_other_crate::learn_sqlx::run();

    test::run();
}
