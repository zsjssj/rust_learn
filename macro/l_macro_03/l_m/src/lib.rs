//官方标准的crate、macro项目结构

pub trait MyDebug {
    fn my_debug(&self) -> String;
}

#[cfg(feature = "derive")]
pub use my_debug_derive::MyDebug;
