use super::Solution;
use std::collections::HashMap;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut idx: HashMap<i32, usize> = HashMap::new(); // 创建一个空哈希表
        for (j, &x) in nums.iter().enumerate() {
            if let Some(&i) = idx.get(&(target - x)) {
                return vec![i as i32, j as i32]; // 返回两个数的下标
            }
            idx.insert(x, j); // 保存 nums[j] 和 j
        }
        unreachable!() // 题目保证有解，循环中一定会 return
    }
}
