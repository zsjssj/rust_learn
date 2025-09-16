#![allow(unused)]

pub fn test() {}

struct Color(u8, u8, u8, f32);
impl Color {
    fn rgb(&self) -> (u8, u8, u8, f32) {
        (self.0, self.1, self.2, self.3)
    }
}
