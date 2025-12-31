//! learn_algorithm模块用于学习各种算法题目，涵盖简单、中等和困难级别的题目。
//! 主要是leedcode上的算法题目练习。
pub mod learn_01; //简单算法题
pub mod learn_02; //中等算法题
pub mod learn_03; //困难算法题
pub mod learn_hot100; //leetcode热题100
pub mod learn_interview150; //面试150题

//算法题目解答结构体
pub struct Solution;

pub fn run() {
    let mut nums = vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4];
    let result = Solution::remove_duplicates(&mut nums);
    println!("{:?}", result);
    println!("{:?}", nums);
}
