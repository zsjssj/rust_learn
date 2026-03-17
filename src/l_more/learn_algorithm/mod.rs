//! learn_algorithm模块用于学习各种算法题目，涵盖简单、中等和困难级别的题目。
//! 主要是leedcode上的算法题目练习。

pub mod learn_01; //简单
pub mod learn_02; //中等
pub mod learn_03; //困难
pub mod learn_hot100; //热100
pub mod learn_interview150; //面150

//算法题目解答结构体
pub struct Solution;

pub fn run() {
    let mut nums = vec![vec![1, 2, 3, 3, 3], vec![4, 0, 6, 5, 7], vec![7, 8, 9, 11, 23]];
    Solution::set_zeroes(&mut nums);
    println!("result: {:?}", nums);
}
