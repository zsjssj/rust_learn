#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

//简单算法题
use super::Solution;
use std::collections::HashMap;

impl Solution {
    //1.两数之和 【空间换时间，哈希表】
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
    //9.回文数【字符串反转】
    pub fn is_palindrome(x: i32) -> bool {
        if x < 0 {
            return false;
        }
        let s: String = x.to_string();
        let rev_s: String = s.chars().rev().collect();
        s == rev_s
    }
    //13.罗马数字转整数【哈希表】
    pub fn roman_to_int(s: String) -> i32 {
        let roman_map: HashMap<char, i32> = HashMap::from([('I', 1), ('V', 5), ('X', 10), ('L', 50), ('C', 100), ('D', 500), ('M', 1000)]);
        let chars: Vec<char> = s.chars().collect();
        let mut total: i32 = 0;
        let n: usize = chars.len();
        for i in 0..n {
            let current_value: i32 = *roman_map.get(&chars[i]).unwrap();
            if i + 1 < n {
                let next_value: i32 = *roman_map.get(&chars[i + 1]).unwrap();
                if current_value < next_value {
                    total -= current_value;
                } else {
                    total += current_value;
                }
            } else {
                total += current_value;
            }
        }
        total
    }
    //14.最长公共前缀【水平扫描】，因为nth是从头遍历 UTF-8 字符，对比之下有点慢
    // pub fn longest_common_prefix(strs: Vec<String>) -> String {
    //     if strs.is_empty() {
    //         return String::new();
    //     }
    //     let mut prefix: String = String::new();
    //     let first_str: &String = &strs[0];
    //     for (i, ch) in first_str.chars().enumerate() {
    //         for other_str in &strs[1..] {
    //             if i >= other_str.len() || other_str.chars().nth(i).unwrap() != ch {
    //                 return prefix;
    //             }
    //         }
    //         prefix.push(ch);
    //     }
    //     prefix
    // }
    //14.最长公共前缀【纵向扫描】快一点点
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        let mut len: usize = strs[0].len();
        let mut prefix: &str = &strs[0][0..len];
        for s in &strs[1..] {
            //逐字节比较前缀（C 级别性能）
            while !s.starts_with(prefix) {
                len -= 1;
                prefix = &strs[0][0..len];
            }
            if prefix.len() == 0 {
                break;
            }
        }
        return String::from(prefix);
    }
    //20.有效的括号【栈】
    pub fn is_valid(s: String) -> bool {
        let mut stack: Vec<char> = Vec::new();
        let bracket_map: HashMap<char, char> = HashMap::from([(')', '('), (']', '['), ('}', '{')]);
        for ch in s.chars() {
            if bracket_map.values().any(|&v| v == ch) {
                stack.push(ch);
            } else if bracket_map.contains_key(&ch) {
                if let Some(top) = stack.pop() {
                    if top != *bracket_map.get(&ch).unwrap() {
                        return false;
                    }
                } else {
                    return false;
                }
            }
        }
        stack.is_empty()
    }
    //21.合并两个有序链表【递归】
    pub fn merge_two_lists(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        match (l1, l2) {
            (Some(mut node1), Some(mut node2)) => {
                if node1.val < node2.val {
                    let next = node1.next.take();
                    node1.next = Self::merge_two_lists(next, Some(node2));
                    Some(node1)
                } else {
                    let next = node2.next.take();
                    node2.next = Self::merge_two_lists(Some(node1), next);
                    Some(node2)
                }
            }
            (Some(node1), None) => Some(node1),
            (None, Some(node2)) => Some(node2),
            (None, None) => None,
        }
    }
    //26.删除排序数组中的重复项【双指针】
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        if nums.is_empty() {
            return 0;
        }
        let mut slow: usize = 0;
        for fast in 1..nums.len() {
            if nums[fast] != nums[slow] {
                slow += 1;
                nums[slow] = nums[fast];
            }
        }
        (slow + 1) as i32
    }
    //27.移除元素【双指针】
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        let mut slow: usize = 0;
        for fast in 0..nums.len() {
            if nums[fast] != val {
                nums[slow] = nums[fast];
                slow += 1;
            }
        }
        slow as i32
    }
    //28.找出字符串中的第一个匹配项的下标【双指针】
    pub fn str_str(haystack: String, needle: String) -> i32 {
        if needle.is_empty() {
            return 0;
        }
        let haystack_bytes: Vec<u8> = haystack.as_bytes().to_vec();
        let needle_bytes: Vec<u8> = needle.as_bytes().to_vec();
        let haystack_len: usize = haystack_bytes.len();
        let needle_len: usize = needle_bytes.len();
        for i in 0..=(haystack_len - needle_len) {
            let mut j: usize = 0;
            while j < needle_len && haystack_bytes[i + j] == needle_bytes[j] {
                j += 1;
            }
            if j == needle_len {
                return i as i32;
            }
        }
        -1
    }
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}
impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}
