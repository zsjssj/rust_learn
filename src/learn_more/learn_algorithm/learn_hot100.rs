//! leetcode热题100算法题解答

#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
use super::Solution;

impl Solution {
    //  1.哈希
    //  1.1两数之和【属于简单，已有】
    pub fn two_sum1(nums: Vec<i32>, target: i32) -> Vec<i32> {
        use std::collections::HashMap;
        let mut m1 = HashMap::<i32, usize>::new();
        for (index, &num) in nums.iter().enumerate() {
            if let Some(&v) = m1.get(&(target - num)) {
                return vec![v as i32, index as i32];
            }
            m1.insert(num, index);
        }
        vec![]
    }
    //  1.2字母异位词分组【中等】
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        use std::collections::HashMap;
        let mut map: HashMap<String, Vec<String>> = HashMap::new();
        for s in strs.into_iter() {
            let mut key_chars: Vec<char> = s.chars().collect();
            key_chars.sort_unstable();
            let key: String = key_chars.into_iter().collect();
            map.entry(key).or_default().push(s);
        }
        map.into_values().collect()
    }
}
