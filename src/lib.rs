//纯库项目入口文件【例如：序列化库】,本库、子库等不会被打包成二进制文件，主要是提供可复用的代码库
pub mod learn_06_10;
pub mod learn_11_15;

pub fn try_some() {
    println!("lib run end: -_-");
    crate::learn_06_10::learn_10::learn_live::test();
}
