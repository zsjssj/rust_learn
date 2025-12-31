/*
  采用发布配置自定义构建
  参考：https://kaisery.github.io/trpl-zh-cn/ch14-01-release-profiles.html
*/

/*

Cargo.toml 示例配置：
  [profile.dev]
    opt-level = 0  # 优化等级，0-3，s，z
  [profile.release]
    opt-level = 3

*/

pub fn run() {
    println!("learn_14_cargoio run");
}
