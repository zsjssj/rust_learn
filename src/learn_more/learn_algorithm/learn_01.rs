#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

//简单算法题
use super::Solution;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

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
        if needle_len > haystack_len {
            return -1;
        }
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
    //35.搜索插入位置【二分查找】
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        let mut left: usize = 0;
        let mut right: usize = nums.len();
        while left < right {
            let mid: usize = left + (right - left) / 2;
            if nums[mid] == target {
                return mid as i32;
            } else if nums[mid] < target {
                left = mid + 1;
            } else {
                right = mid;
            }
        }
        left as i32
    }
    //58.最后一个单词的长度【字符串处理】
    pub fn length_of_last_word(s: String) -> i32 {
        let trimmed: &str = s.trim_end();
        if let Some(pos) = trimmed.rfind(' ') {
            (trimmed.len() - pos - 1) as i32
        } else {
            trimmed.len() as i32
        }
    }
    //66.加一【模拟加法器】
    /*
       给定一个表示 大整数 的整数数组 digits，其中 digits[i] 是整数的第 i 位数字。这些数字按从左到右，从最高位到最低位排列。这个大整数不包含任何前导 0。
       将大整数加 1，并返回结果的数字数组。
    */
    pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
        let mut result: Vec<i32> = digits.clone();
        let n: usize = result.len();
        for i in (0..n).rev() {
            if result[i] < 9 {
                result[i] += 1;
                return result;
            }
            result[i] = 0;
        }
        let mut new_result: Vec<i32> = Vec::with_capacity(n + 1);
        new_result.push(1);
        new_result.extend(result);
        new_result
    }
    //67.二进制求和【模拟二进制加法器】
    pub fn add_binary(a: String, b: String) -> String {
        let mut result: String = String::new(); // 存储结果
        let mut carry: i32 = 0; // 进位
        let a_bytes: Vec<u8> = a.as_bytes().to_vec(); // 将字符串转换为字节数组
        let b_bytes: Vec<u8> = b.as_bytes().to_vec();
        let mut i: isize = (a_bytes.len() as isize) - 1; // 从最后一位开始遍历
        let mut j: isize = (b_bytes.len() as isize) - 1;
        while i >= 0 || j >= 0 || carry > 0 {
            let mut sum: i32 = carry; // 当前位的和，初始值为进位
            if i >= 0 {
                sum += (a_bytes[i as usize] - b'0') as i32; // 转换为数字并加到 sum 上
                i -= 1;
            }
            if j >= 0 {
                sum += (b_bytes[j as usize] - b'0') as i32;
                j -= 1;
            }
            result.push(((sum % 2) as u8 + b'0') as char);
            carry = sum / 2;
        }
        result.chars().rev().collect()
    }
    //69.x 的平方根【二分查找】
    pub fn my_sqrt(x: i32) -> i32 {
        if x < 2 {
            return x;
        }
        let mut left: i32 = 1;
        let mut right: i32 = x / 2;
        while left <= right {
            let mid: i32 = left + (right - left) / 2;
            let mid_squared: i64 = (mid as i64) * (mid as i64);
            if mid_squared == x as i64 {
                return mid;
            } else if mid_squared < x as i64 {
                left = mid + 1;
            } else {
                right = mid - 1;
            }
        }
        right
    }
    //70.爬楼梯【动态规划】
    pub fn climb_stairs(n: i32) -> i32 {
        if n <= 2 {
            return n;
        }
        let mut dp: Vec<i32> = vec![0; (n + 1) as usize];
        dp[1] = 1;
        dp[2] = 2;
        for i in 3..=n as usize {
            dp[i] = dp[i - 1] + dp[i - 2];
        }
        dp[n as usize]
    }
    //83.删除排序链表中的重复元素【迭代】优化版【更快（原地修改）】
    pub fn delete_duplicates(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        if head.is_none() {
            return None;
        }
        let mut curr = head.as_mut();
        while let Some(node_curr) = curr {
            while node_curr.next.is_some() && node_curr.next.as_ref().unwrap().val == node_curr.val {
                node_curr.next = node_curr.next.as_mut().unwrap().next.take();
            }
            curr = node_curr.next.as_mut();
        }
        head
    }
    //88.合并两个有序数组【双指针从后向前】
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &Vec<i32>, n: i32) {
        let mut p1: isize = (m - 1) as isize;
        let mut p2: isize = (n - 1) as isize;
        let mut p: isize = (m + n - 1) as isize;
        while p1 >= 0 && p2 >= 0 {
            if nums1[p1 as usize] > nums2[p2 as usize] {
                nums1[p as usize] = nums1[p1 as usize];
                p1 -= 1;
            } else {
                nums1[p as usize] = nums2[p2 as usize];
                p2 -= 1;
            }
            p -= 1;
        }
        while p2 >= 0 {
            nums1[p as usize] = nums2[p2 as usize];
            p2 -= 1;
            p -= 1;
        }
    }
    //94.二叉树的中序遍历【迭代】
    pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut result: Vec<i32> = Vec::new();
        let mut stack: Vec<Rc<RefCell<TreeNode>>> = Vec::new();
        let mut current: Option<Rc<RefCell<TreeNode>>> = root;
        while current.is_some() || !stack.is_empty() {
            while let Some(node) = current {
                stack.push(Rc::clone(&node));
                current = node.borrow().left.clone();
            }
            if let Some(node) = stack.pop() {
                result.push(node.borrow().val);
                current = node.borrow().right.clone();
            }
        }
        result
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

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}
impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode { val, left: None, right: None }
    }
}
