//! 顾及不同类型值的 trait 对象
//参考：https://kaisery.github.io/trpl-zh-cn/ch18-01-what-is-oo.html
//对象包含数据和行为

pub fn run() {
    test1();
}

//定义通用行为的 trait
fn test1() {}
pub trait Draw {
    fn draw(&self);
}
pub struct Screen<T: Draw> {
    pub components: Vec<T>,
}

impl<T> Screen<T>
where
    T: Draw,
{
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}
