//纯库项目入口文件【例如：序列化库】,本库、子库等不会被打包成二进制文件，主要是提供可复用的代码库
pub mod collections;
pub mod learn_enum;
pub mod learn_error_handle;

pub fn try_some() {
    println!("lib run");
    crate::learn_error_handle::learn_panic::test();
}
