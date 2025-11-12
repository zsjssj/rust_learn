//! 简单算法题模块
//!
//! 默认leedcode的简单算法题顺序的解答
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

use super::Solution;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

impl Solution {
    ///1.两数之和【空间换时间，哈希表】
    ///
    /// 给定一个整数数组 nums 和一个整数目标值 target，请你在该数组中找出 和为目标值 target 的那 两个 整数，并返回它们的数组下标。
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
    ///9.回文数【字符串反转】
    ///
    /// 给你一个整数 x ，如果 x 是一个回文整数，返回 true ；否则，返回 false 。
    ///
    /// 回文数是指正序（从左向右）和倒序（从右向左）读都是一样的整数。
    pub fn is_palindrome(x: i32) -> bool {
        if x < 0 {
            return false;
        }
        let s: String = x.to_string();
        let rev_s: String = s.chars().rev().collect();
        s == rev_s
    }
    ///13.罗马数字转整数【哈希表】
    ///
    /// 将罗马数字转换为整数。输入为一个表示罗马数字的字符串 s ，输出其对应的整数值。
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
    ///14.最长公共前缀【纵向扫描】快一点点
    ///
    /// 编写一个函数来查找字符串数组中的最长公共前缀。如果不存在公共前缀，返回空字符串 ""。
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
    ///20.有效的括号【栈】
    ///
    /// 给定一个只包括 '('，')'，'{'，'}'，'['，']' 的字符串 s ，判断字符串是否有效。
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
    ///21.合并两个有序链表【递归】
    ///
    /// 将两个升序链表合并为一个新的 升序 链表并返回。新链表是通过拼接给定的两个链表的所有节点组成的。
    pub fn merge_two_lists(l1: List, l2: List) -> List {
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
    ///26.删除排序数组中的重复项【双指针】
    ///
    /// 给你一个有序数组 nums ，请你 原地 删除重复出现的元素，使每个元素 只出现一次 ，返回删除后数组的新长度。
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
    ///27.移除元素【双指针】
    ///
    /// 给你一个数组 nums 和一个值 val，你需要 原地 移除所有数值等于 val 的元素，并返回移除后数组的新长度。
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
    ///28.找出字符串中的第一个匹配项的下标【双指针】
    ///
    /// 实现 strStr() 函数。给你两个字符串 haystack 和 needle ，请你在 haystack 字符串中找出 needle 字符串的第一个匹配项的下标（下标从 0 开始）。如果 needle 不是 haystack 的一部分，则返回 -1 。
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
    ///35.搜索插入位置【二分查找】
    ///
    /// 给定一个排序数组 nums 和一个目标值 target ，请你在数组中找到目标值，并返回其下标。如果目标值不存在于数组中，返回它将会被按顺序插入的位置。
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
    ///58.最后一个单词的长度【字符串处理】
    ///
    /// 给你一个字符串 s ，由若干单词组成，单词之间由若干空格隔开。返回字符串中最后一个单词的长度。
    pub fn length_of_last_word(s: String) -> i32 {
        let trimmed: &str = s.trim_end();
        if let Some(pos) = trimmed.rfind(' ') {
            (trimmed.len() - pos - 1) as i32
        } else {
            trimmed.len() as i32
        }
    }
    ///66.加一【模拟加法器】
    ///
    ///给定一个表示 大整数 的整数数组 digits，其中 digits[i] 是整数的第 i 位数字。这些数字按从左到右，从最高位到最低位排列。
    ///
    /// 这个大整数不包含任何前导 0。 将大整数加 1，并返回结果的数字数组。
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
    ///67.二进制求和【模拟二进制加法器】
    ///
    /// 给你两个二进制字符串，返回它们的和（用二进制表示）。
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
    ///69.x 的平方根【二分查找】
    ///
    /// 给你一个非负整数 x ，计算并返回 x 的 算术平方根 。由于返回类型是整数，结果只保留 整数部分 ，小数部分将被舍去 。
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
    ///70.爬楼梯【动态规划】
    ///
    /// 假设你正在爬楼梯。需要 n 阶你才能到达楼顶。 每次你可以爬 1 或 2 个台阶。你有多少种不同的方法可以爬到楼顶呢？
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
    ///83.删除排序链表中的重复元素【迭代】优化版【更快（原地修改）】
    ///
    /// 给定一个排序链表，删除所有重复的元素，使每个元素只出现一次。返回已排序的链表。
    pub fn delete_duplicates(mut head: List) -> List {
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
    ///88.合并两个有序数组【双指针从后向前】
    ///
    /// 给你两个有序整数数组 nums1 和 nums2，请你将 nums2 合并到 nums1 中，使 nums1 成为一个有序数组。
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
    ///94.二叉树的中序遍历【迭代】
    ///
    /// 给定一个二叉树的根节点 root ，返回它的 中序 遍历。
    pub fn inorder_traversal(root: Tree) -> Vec<i32> {
        let mut result: Vec<i32> = Vec::new();
        let mut stack: Vec<Rc<RefCell<TreeNode>>> = Vec::new();
        let mut current: Tree = root;
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
    ///100.相同的树【递归】
    ///   
    /// 给你两棵二叉树的根节点 p 和 q ，编写一个函数来检验这两棵树是否相同。
    pub fn is_same_tree(p: Tree, q: Tree) -> bool {
        match (p, q) {
            (Some(node_p), Some(node_q)) => {
                let p_borrow = node_p.borrow();
                let q_borrow = node_q.borrow();
                p_borrow.val == q_borrow.val
                    && Self::is_same_tree(p_borrow.left.clone(), q_borrow.left.clone())
                    && Self::is_same_tree(p_borrow.right.clone(), q_borrow.right.clone())
            }
            (None, None) => true,
            _ => false,
        }
    }
    ///101. 对称二叉树
    ///
    /// 给你一个二叉树的根节点 root ， 检查它是否轴对称。
    pub fn is_symmetric(root: Tree) -> bool {
        fn is_mirror(t1: Tree, t2: Tree) -> bool {
            match (t1, t2) {
                (Some(node1), Some(node2)) => {
                    let n1 = node1.borrow();
                    let n2 = node2.borrow();
                    n1.val == n2.val && is_mirror(n1.left.clone(), n2.right.clone()) && is_mirror(n1.right.clone(), n2.left.clone())
                }
                (None, None) => true,
                _ => false,
            }
        }
        is_mirror(root.clone(), root)
    }
    ///104.二叉树的最大深度【递归】
    ///
    /// 给定一个二叉树的根节点 root ，返回其最大深度。
    pub fn max_depth(root: Tree) -> i32 {
        match root {
            Some(node) => {
                let n = node.borrow();
                1 + std::cmp::max(Self::max_depth(n.left.clone()), Self::max_depth(n.right.clone()))
            }
            None => 0,
        }
    }
    ///108.将有序数组转换为二叉搜索树【递归】
    ///   
    /// 给你一个整数数组 nums ，其中元素已经按 升序 排列，请你将其转换为一棵 高度平衡 二叉搜索树。
    pub fn sorted_array_to_bst(nums: Vec<i32>) -> Tree {
        fn build_bst(nums: &Vec<i32>, left: isize, right: isize) -> Tree {
            if left > right {
                return None;
            }
            let mid: isize = left + (right - left) / 2;
            let node: Rc<RefCell<TreeNode>> = Rc::new(RefCell::new(TreeNode::new(nums[mid as usize])));
            node.borrow_mut().left = build_bst(nums, left, mid - 1);
            node.borrow_mut().right = build_bst(nums, mid + 1, right);
            Some(node)
        }
        build_bst(&nums, 0, (nums.len() as isize) - 1)
    }
    ///110.平衡二叉树
    ///
    /// 给定一个二叉树，判断它是否是高度平衡的二叉树。
    pub fn is_balanced(root: Tree) -> bool {
        fn check_balance(node: &Tree) -> i32 {
            match node {
                Some(n) => {
                    let n_borrow = n.borrow();
                    let left_height = check_balance(&n_borrow.left);
                    if left_height == -1 {
                        return -1;
                    }
                    let right_height = check_balance(&n_borrow.right);
                    if right_height == -1 {
                        return -1;
                    }
                    if (left_height - right_height).abs() > 1 {
                        return -1;
                    }
                    1 + std::cmp::max(left_height, right_height)
                }
                None => 0,
            }
        }
        check_balance(&root) != -1
    }
    ///111.二叉树的最小深度【递归】
    ///   
    /// 给定一个二叉树的根节点 root ，返回其最小深度。
    pub fn min_depth(root: Tree) -> i32 {
        match root {
            Some(node) => {
                let n = node.borrow();
                let left_depth = Self::min_depth(n.left.clone());
                let right_depth = Self::min_depth(n.right.clone());
                if left_depth == 0 || right_depth == 0 {
                    1 + left_depth + right_depth
                } else {
                    1 + std::cmp::min(left_depth, right_depth)
                }
            }
            None => 0,
        }
    }
    ///112.路径总和【递归】
    ///   
    /// 给你二叉树的根节点 root 和一个表示目标和的整数 targetSum ，判断该树中是否存在 根节点到叶子节点 的路径，
    /// 这条路径上所有节点值相加等于目标和 targetSum 。
    pub fn has_path_sum(root: Tree, target_sum: i32) -> bool {
        match root {
            Some(node) => {
                let n = node.borrow();
                let new_target = target_sum - n.val;
                if n.left.is_none() && n.right.is_none() {
                    new_target == 0
                } else {
                    Self::has_path_sum(n.left.clone(), new_target) || Self::has_path_sum(n.right.clone(), new_target)
                }
            }
            None => false,
        }
    }
}

/// 通用链表结构
/// * `val`: i32 - 节点值
/// * `next`: 下一个节点
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: List,
}
type List = Option<Box<ListNode>>;
impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

///二叉树结构
/// * `val`: i32 - 节点值
/// * `left`: 左子节点
/// * `right`: 右子节点
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Tree,
    pub right: Tree,
}
pub type Tree = Option<Rc<RefCell<TreeNode>>>;
impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode { val, left: None, right: None }
    }
}
