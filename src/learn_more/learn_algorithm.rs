pub mod learn_01; //简单算法题
pub mod learn_02; //中等算法题
pub mod learn_03; //困难算法题

pub struct Solution;

pub fn run() {
    let nums = vec![1, 2, 5, 7, 11, 15];
    let target = 12;
    let result = Solution::two_sum(nums, target);
    println!("{:?}", result);
}
