pub mod learn_01_20;

pub struct Solution;

pub fn run() {
    let nums = vec![1, 2, 5, 7, 11, 15];
    let target = 12;
    let result = Solution::two_sum(nums, target);
    println!("{:?}", result);
}
