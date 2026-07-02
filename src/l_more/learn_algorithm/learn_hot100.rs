//! leetcode热题100算法题解答

#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
use super::Solution;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(PartialEq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

pub struct RandomListNode {
    pub label: i32,
    pub next: Option<Box<RandomListNode>>,
    pub random: Option<Box<RandomListNode>>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Tree,
    pub right: Tree,
}
pub type Tree = Option<Rc<RefCell<TreeNode>>>;
impl TreeNode {
    fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

impl Solution {
    //  1.哈希
    //  1.1两数之和【简单】
    ///给定一个整数数组 nums 和一个目标值 target，请你在该数组中找出和为目标值的那 两个 整数，并返回他们的数组下标。
    ///
    ///你可以假设每种输入只会对应一个答案。但是，数组中同一个元素不能使用两遍。
    pub fn two_sum1(nums: Vec<i32>, target: i32) -> Vec<i32> {
        use std::collections::HashMap; // 引入 HashMap
        let mut m1 = HashMap::<i32, usize>::new(); // 存储数值及其索引
        // 遍历数组
        for (index, &num) in nums.iter().enumerate() {
            // 检查是否存在补数
            if let Some(&v) = m1.get(&(target - num)) {
                return vec![v as i32, index as i32]; // 找到目标值，返回索引
            }
            m1.insert(num, index); // 存储当前数值及其索引
        }
        vec![]
    }
    //  1.2字母异位词分组【中等】
    ///给你一个字符串数组，请你将 字母异位词 组合在一起。可以按任意顺序返回结果列表。
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        use std::collections::HashMap;
        let mut map: HashMap<String, Vec<String>> = HashMap::new(); // 存储排序后的字符串及其对应的异位词列表
        // 遍历输入字符串数组
        for s in strs.into_iter() {
            let mut key_chars: Vec<char> = s.chars().collect(); // 将字符串转换为字符向量
            key_chars.sort_unstable(); // 对字符进行排序
            let key: String = key_chars.into_iter().collect(); // 将排序后的字符重新组合成字符串作为键
            map.entry(key).or_default().push(s); // 将原字符串添加到对应的异位词列表中
        }
        map.into_values().collect() // 返回所有异位词组的列表
    }
    //  1.3最长连续序列【中等】
    /// 给定一个未排序的整数数组 nums ，找出数字连续的最长序列（不要求序列元素在原数组中连续）的长度。
    /// 设计并实现时间复杂度为 O(n) 的算法解决此问题。
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        use std::collections::HashSet;
        let set: HashSet<_> = nums.into_iter().collect();
        let mut longest = 0;
        for &num in &set {
            // 只有当 num 是序列起点（num-1 不存在）时才开始计数
            if !set.contains(&(num - 1)) {
                let mut length = 1;
                let mut cur = num;
                while set.contains(&(cur + 1)) {
                    cur += 1;
                    length += 1;
                }
                longest = longest.max(length);
            }
        }
        longest
    }

    //  2.双指针
    //  2.1移动零【简单】
    /// 给定一个数组 nums，编写一个函数将所有 0 移动到数组的末尾，同时保持非零元素的相对顺序。
    /// 请注意，必须在不复制数组的情况下原地对数组进行操作。
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        let mut last_non_zero = 0;
        for i in 0..nums.len() {
            if nums[i] != 0 {
                nums.swap(i, last_non_zero);
                last_non_zero += 1;
            }
        }
    }
    //  2.2盛最多水的容器【中等】
    /// 给定 n 个非负整数 a1，a2，...，an，每个数代表坐标中的一个点 (i, ai) 。
    /// 在坐标内画 n 条垂直线，垂直线 i 的两个端点分别为 (i, ai) 和 (i, 0)。
    /// 找出其中的两条线，使得它们与 x 轴共同构成的容器可以容纳最多的水。
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut left = 0;
        let mut right = height.len() - 1;
        let mut max_area = 0;
        while left < right {
            let h = height[left].min(height[right]);
            let w = (right - left) as i32;
            let area = h * w;
            max_area = max_area.max(area);
            if height[left] < height[right] {
                left += 1;
            } else {
                right -= 1;
            }
        }
        max_area
    }
    //  2.3三数之和【中等】
    /// 给你一个包含 n 个整数的数组 nums，判断 nums 中是否存在三个元素 a，b，c ，使得 a + b + c = 0 ？请你找出所有满足条件且不重复的三元组。
    /// 注意：答案中不可以包含重复的三元组。
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut nums = nums;
        nums.sort_unstable();
        let mut result = Vec::new();
        let n = nums.len();
        for i in 0..n {
            if i > 0 && nums[i] == nums[i - 1] {
                continue; // 跳过重复的第一个元素
            }
            let mut left = i + 1;
            let mut right = n - 1;
            while left < right {
                let sum = nums[i] + nums[left] + nums[right];
                if sum == 0 {
                    result.push(vec![nums[i], nums[left], nums[right]]);
                    while left < right && nums[left] == nums[left + 1] {
                        left += 1; // 跳过重复的第二个元素
                    }
                    while left < right && nums[right] == nums[right - 1] {
                        right -= 1; // 跳过重复的第三个元素
                    }
                    left += 1;
                    right -= 1;
                } else if sum < 0 {
                    left += 1;
                } else {
                    right -= 1;
                }
            }
        }
        result
    }
    //  2.4接雨水【困难】
    /// 给定 n 个非负整数表示每个宽度为 1 的柱子的高度图，计算按此排列的柱子，下雨之后能接多少雨水。
    pub fn trap(height: Vec<i32>) -> i32 {
        let n = height.len();
        if n == 0 {
            return 0;
        }
        let mut left = 0;
        let mut right = n - 1;
        let mut left_max = height[left];
        let mut right_max = height[right];
        let mut water_trapped = 0;
        while left < right {
            if left_max < right_max {
                left += 1;
                left_max = left_max.max(height[left]);
                water_trapped += left_max - height[left];
            } else {
                right -= 1;
                right_max = right_max.max(height[right]);
                water_trapped += right_max - height[right];
            }
        }
        water_trapped
    }

    //  3.滑动窗口
    //  3.1无重复字符的最长子串【中等】
    /// 给定一个字符串 s ，请你找出其中不含有重复字符的 最长子串 的长度。
    pub fn length_of_longest_substring(s: String) -> i32 {
        use std::collections::HashMap;
        let mut char_index_map = HashMap::new(); // 存储字符及其最新索引
        let (mut left, mut max_length) = (0, 0); // 左指针和最大长度
        for (right, c) in s.chars().enumerate() {
            //判断有无重复字符
            if let Some(&prev_index) = char_index_map.get(&c) {
                //判断重复字符【old】是否在当前窗口内,是就移动左指针【窗口左侧移动到重复字符【old】的下一个位置】
                if prev_index >= left {
                    left = prev_index + 1; //移动左指针
                }
            }
            char_index_map.insert(c, right);
            max_length = max_length.max(right - left + 1);
        }
        max_length as i32
    }
    //  3.2找到字符串中所有字母异位词【中等】
    /// 给定两个字符串 s 和 p ，找到 s 中所有是 p 的 异位词 的子串，返回这些子串的起始索引。不考虑答案输出的顺序。
    /// 最快速解法：滑动窗口 + 计数数组
    pub fn find_anagrams(s: String, p: String) -> Vec<i32> {
        let s_bytes = s.as_bytes();
        let p_bytes = p.as_bytes();
        let p_len = p_bytes.len();
        let s_len = s_bytes.len();
        if s_len < p_len {
            return vec![];
        }
        let mut p_count = [0; 26];
        let mut s_count = [0; 26];
        for &b in p_bytes {
            p_count[(b - b'a') as usize] += 1;
        }
        let mut result = Vec::new();
        for i in 0..s_len {
            s_count[(s_bytes[i] - b'a') as usize] += 1;
            if i >= p_len {
                s_count[(s_bytes[i - p_len] - b'a') as usize] -= 1;
            }
            if s_count == p_count {
                result.push((i + 1 - p_len) as i32);
            }
        }
        result
    }
    fn find_anagrams2(s: String, p: String) -> Vec<i32> {
        use std::collections::HashMap;
        let mut result = Vec::new();
        let (s_len, p_len) = (s.len(), p.len());
        if s_len < p_len {
            return result;
        }
        let mut p_count = HashMap::new();
        let mut s_count = HashMap::new();
        for c in p.chars() {
            *p_count.entry(c).or_insert(0) += 1; //统计p中字符出现次数
        }
        let s_chars: Vec<char> = s.chars().collect();
        for i in 0..s_len {
            *s_count.entry(s_chars[i]).or_insert(0) += 1;
            if i >= p_len {
                let left_char = s_chars[i - p_len];
                if let Some(count) = s_count.get_mut(&left_char) {
                    *count -= 1;
                    if *count == 0 {
                        s_count.remove(&left_char);
                    }
                }
            }
            if s_count == p_count {
                result.push((i + 1 - p_len) as i32);
            }
        }
        result
    }

    //  4.子串
    //  4.1和为K的子数组【中等】
    /// 给定一个整数数组和一个整数 k ，你需要找到该数组中和为 k 的连续子数组的个数。
    pub fn subarray_sum(nums: Vec<i32>, k: i32) -> i32 {
        use std::collections::HashMap;
        let mut count = 0; // 子数组个数
        let mut sum = 0; // 当前前缀和
        let mut prefix_sums = HashMap::new(); // 存储前缀和及其出现次数
        prefix_sums.insert(0, 1); // 前缀和为0的情况
        for &num in &nums {
            sum += num;
            if let Some(&occurrences) = prefix_sums.get(&(sum - k)) {
                count += occurrences;
            }
            *prefix_sums.entry(sum).or_insert(0) += 1;
        }
        count
    }
    // 最快解法:
    fn subarray_sum2(nums: Vec<i32>, k: i32) -> i32 {
        use std::collections::HashMap;
        let mut s = vec![0; nums.len() + 1];
        for (index, &x) in nums.iter().enumerate() {
            s[index + 1] = s[index] + x;
        }
        let mut cnt = HashMap::with_capacity(s.len());
        let mut ans = 0;
        for sj in s {
            if let Some(&c) = cnt.get(&(sj - k)) {
                ans += c;
            }
            *cnt.entry(sj).or_insert(0) += 1;
        }
        ans
    }
    //  4.2窗口滑动中的最大值【困难】
    /// 给你一个整数数组 nums，有一个大小为 k 的滑动窗口从数组的最左侧移动到数组的最右侧。
    /// 你只可以看到在滑动窗口内的 k 个数字。滑动窗口每次只向右移动一位。
    /// 返回 滑动窗口中的最大值 。
    pub fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
        use std::collections::VecDeque;
        let n = nums.len();
        if n == 0 || k as usize > n {
            return vec![];
        }
        let k = k as usize;
        let mut result = Vec::with_capacity(n - k + 1);
        let mut deque: VecDeque<usize> = VecDeque::new(); // 存储索引
        for i in 0..n {
            // 移除不在窗口内的元素索引
            if let Some(&front) = deque.front() {
                if front + k <= i {
                    deque.pop_front();
                }
            }
            // 移除所有小于当前元素的索引
            while let Some(&back) = deque.back() {
                if nums[back] < nums[i] {
                    deque.pop_back();
                } else {
                    break;
                }
            }
            deque.push_back(i);
            // 当窗口达到大小k时，记录当前最大值
            if i + 1 >= k {
                if let Some(&front) = deque.front() {
                    result.push(nums[front]);
                }
            }
        }
        result
    }
    //  4.3最小覆盖子串【困难】
    /// 给你一个字符串 S、一个字符串 T 。返回 S 中涵盖 T 所有字符的最小子串。
    /// 如果 S 中不存在涵盖 T 所有字符的子串，返回空字符串 "" 。
    pub fn min_window(s: String, t: String) -> String {
        use std::collections::HashMap;
        let mut need = HashMap::new();
        let mut window = HashMap::new();
        for c in t.chars() {
            *need.entry(c).or_insert(0) += 1;
        }
        let (mut left, mut right) = (0, 0);
        let (mut valid, mut start, mut len) = (0, 0, usize::MAX);
        let s_chars: Vec<char> = s.chars().collect();
        while right < s_chars.len() {
            let c = s_chars[right];
            right += 1;
            if need.contains_key(&c) {
                *window.entry(c).or_insert(0) += 1;
                if window[&c] == need[&c] {
                    valid += 1;
                }
            }
            while valid == need.len() {
                if right - left < len {
                    start = left;
                    len = right - left;
                }
                let d = s_chars[left];
                left += 1;
                if need.contains_key(&d) {
                    if window[&d] == need[&d] {
                        valid -= 1;
                    }
                    *window.entry(d).or_insert(0) -= 1;
                }
            }
        }
        if len == usize::MAX {
            "".to_string()
        } else {
            s_chars[start..start + len].iter().collect()
        }
    }

    //  5.普通数组
    //  5.1 最大子数组和【中等】
    /// 给你一个整数数组 nums ，请你找出一个具有最大和的连续子数组（子数组最少包含一个元素），返回其最大和。
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let mut max_current = nums[0];
        let mut max_global = nums[0];
        for &num in nums.iter().skip(1) {
            max_current = max_current.max(0) + num; // 如果 max_current 为负数，则从当前元素重新开始
            if max_current > max_global {
                max_global = max_current;
            }
        }
        max_global
    }
    //  5.2合并区间【中等】
    ///以数组 intervals 表示若干个区间的集合，其中单个区间为 intervals[i] = [starti, endi] 。
    ///请你合并所有重叠的区间，并返回一个不重叠的区间数组，该数组需恰好覆盖输入中的所有区间。
    pub fn merge(intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut intervals: Vec<Vec<i32>> = intervals;
        intervals.sort_unstable_by_key(|interval| interval[0]);
        let mut merge_arr: Vec<Vec<i32>> = Vec::new();
        for interval in intervals {
            if let Some(last) = merge_arr.last_mut() {
                if interval[0] <= last[1] {
                    last[1] = last[1].max(interval[1]);
                } else {
                    merge_arr.push(interval);
                }
            } else {
                merge_arr.push(interval);
            }
        }
        merge_arr
    }
    //  5.3轮转数组【中等】
    /// 给定一个数组，将数组中的元素向右轮转 k 个位置，其中 k 是非负数。
    pub fn rotate(nums: &mut Vec<i32>, k: i32) {
        let n = nums.len();
        let k = (k as usize) % n;
        nums.reverse();
        nums[..k].reverse();
        nums[k..].reverse();
    }
    //  5.4除了自己以外数组的乘积【中等】
    /// 给你一个长度为 n 的整数数组 nums，其中 n > 1，返回一个输出数组 output，其中 output[i] 等于 nums 中除了 nums[i] 以外其余各元素的乘积。
    /// 题目数据保证数组 nums 中任意元素的乘积都在 32 位整数范围内。
    /// 请不要使用除法，且在 O(n) 时间复杂度内完成此题。
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        let mut output = vec![1; n];
        let mut left_product = 1;
        for i in 0..n {
            output[i] = left_product;
            left_product *= nums[i];
        }
        let mut right_product = 1;
        for i in (0..n).rev() {
            output[i] *= right_product;
            right_product *= nums[i];
        }
        output
    }
    pub fn product_except_self2(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        let mut pre = vec![1; n];
        for i in 1..n {
            pre[i] = pre[i - 1] * nums[i - 1];
        }
        let mut suf = vec![1; n];
        for i in (0..n - 1).rev() {
            suf[i] = suf[i + 1] * nums[i + 1];
        }
        pre.iter().zip(suf.iter()).map(|(&p, &s)| p * s).collect()
    }
    //  5.4缺失的第一个正数【困难】
    /// 给你一个未排序的整数数组 nums ，请你找出其中没有出现的最小的正整数。
    /// 请你实现时间复杂度为 O(n) 并且只使用常数级空间的解决方案。
    pub fn first_missing_positive(nums: &mut Vec<i32>) -> i32 {
        let n = nums.len();
        for i in 0..n {
            while nums[i] > 0 && (nums[i] as usize) <= n && nums[nums[i] as usize - 1] != nums[i] {
                let target_index = nums[i] as usize - 1;
                nums.swap(i, target_index);
            }
        }
        for i in 0..n {
            if nums[i] != (i as i32 + 1) {
                return i as i32 + 1;
            }
        }
        (n as i32) + 1
    }

    //  6.矩阵
    //  6.1矩阵置零【中等】
    /// 给定一个 m x n 的矩阵，如果一个元素为 0 ，则将其所在行和列的所有元素都设为 0 。请使用 原地 算法。
    pub fn set_zeroes(matrix: &mut Vec<Vec<i32>>) {
        let m = matrix.len();
        let n = matrix[0].len();
        let mut first_row_zero = false;
        let mut first_col_zero = false;
        for i in 0..m {
            if matrix[i][0] == 0 {
                first_col_zero = true;
                break;
            }
        }
        for j in 0..n {
            if matrix[0][j] == 0 {
                first_row_zero = true;
                break;
            }
        }
        for i in 1..m {
            for j in 1..n {
                if matrix[i][j] == 0 {
                    matrix[i][0] = 0;
                    matrix[0][j] = 0;
                }
            }
        }
        for i in 1..m {
            for j in 1..n {
                if matrix[i][0] == 0 || matrix[0][j] == 0 {
                    matrix[i][j] = 0;
                }
            }
        }
        if first_row_zero {
            for j in 0..n {
                matrix[0][j] = 0;
            }
        }
        if first_col_zero {
            for i in 0..m {
                matrix[i][0] = 0;
            }
        }
    }
    //  6.2螺旋矩阵【中等】
    /// 给你一个 m 行 n 列的矩阵 matrix ，请按照 顺时针螺旋顺序 ，返回矩阵中的所有元素。
    pub fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
        let mut result = Vec::new();
        if matrix.is_empty() {
            return result;
        }
        let (mut top, mut bottom) = (0, matrix.len() - 1);
        let (mut left, mut right) = (0, matrix[0].len() - 1);
        while top <= bottom && left <= right {
            for j in left..=right {
                result.push(matrix[top][j]);
            }
            top += 1;
            for i in top..=bottom {
                result.push(matrix[i][right]);
            }
            if top <= bottom {
                right = right.saturating_sub(1);
                for j in (left..=right).rev() {
                    result.push(matrix[bottom][j]);
                }
                bottom = bottom.saturating_sub(1);
            }
            if left <= right {
                for i in (top..=bottom).rev() {
                    result.push(matrix[i][left]);
                }
                left += 1;
            }
        }
        result
    }
    //  6.3 旋转图像【中等】
    /// 给定一个 n × n 的二维矩阵 matrix 表示一个图像。
    /// 请你将图像顺时针旋转 90 度。你必须在 原地 旋转图像，这意味着你需要直接修改输入的二维矩阵。请不要 使用另一个矩阵来旋转图像。
    pub fn rotate_matrix(matrix: &mut Vec<Vec<i32>>) {
        let n = matrix.len();
        for i in 0..n {
            for j in i..n {
                matrix[i][j] ^= matrix[j][i];
                matrix[j][i] ^= matrix[i][j];
                matrix[i][j] ^= matrix[j][i];
            }
        }
        for i in 0..n {
            for j in 0..n / 2 {
                matrix[i][j] ^= matrix[i][n - 1 - j];
                matrix[i][n - 1 - j] ^= matrix[i][j];
                matrix[i][j] ^= matrix[i][n - 1 - j];
            }
        }
    }
    //  6.4搜索二维矩阵【中等】
    /// 编写一个高效的算法来判断 m x n 矩阵中是否存在一个目标值。该矩阵具有如下特性：
    /// 每行中的整数从左到右按升序排列。
    /// 每行的第一个整数大于前一行的最后一个整数。
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        if matrix.is_empty() || matrix[0].is_empty() {
            return false;
        }
        let m = matrix.len();
        let n = matrix[0].len();
        let mut left = 0;
        let mut right = m * n - 1;
        while left <= right {
            let mid = left + (right - left) / 2;
            let mid_value = matrix[mid / n][mid % n];
            if mid_value == target {
                return true;
            } else if mid_value < target {
                left = mid + 1;
            } else {
                right = mid - 1;
            }
        }
        false
    }

    //  7链表
    //  7.1 相交链表【简单】
    /// 给你两个单链表的头节点 headA 和 headB ，请你找出并返回两个单链表相交的起始节点。如果两个链表没有交点，返回 null 。
    // pub fn get_intersection_node(
    //     head_a: Option<Box<ListNode>>,
    //     head_b: Option<Box<ListNode>>,
    // ) -> Option<Box<ListNode>> {
    //     let mut a = head_a;
    //     let mut b = head_b;
    //     while a != b {
    //         a = if let Some(node) = a { node.next } else { head_b.clone() };
    //         b = if let Some(node) = b { node.next } else { head_a.clone() };
    //     }
    //     a
    // }
    ///
    //  7.2 反转链表
    /// 给你单链表的头节点 head ，请你反转链表，并返回反转后的链表。
    pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut prev = None;
        let mut current = head;
        while let Some(mut node) = current {
            let next = node.next.take(); // 保存下一个节点
            node.next = prev; // 反转当前节点的指针
            prev = Some(node); // 移动 prev 到当前节点
            current = next; // 移动 current 到下一个节点
        }
        prev
    }
    //  7.3 回文链表
    /// 给你一个单链表的头节点 head ，请你判断该链表是否为回文链表。如果是，返回 true ；否则，返回 false 。
    pub fn is_palindrome(head: Option<Box<ListNode>>) -> bool {
        let mut slow = head.as_ref();
        let mut fast = head.as_ref();
        let mut stack = Vec::new();
        while let Some(s) = slow {
            stack.push(s.val);
            slow = s.next.as_ref();
            if let Some(f) = fast {
                fast = f.next.as_ref().and_then(|node| node.next.as_ref());
            } else {
                break;
            }
        }
        while let Some(s) = slow {
            if stack.pop() != Some(s.val) {
                return false;
            }
            slow = s.next.as_ref();
        }
        true
    }
    //  7.4环形链表【简单】
    /// 给定一个链表，判断链表中是否有环。
    /// 如果链表中存在环，则返回 true 。 否则，返回 false 。
    pub fn has_cycle(head: Option<Box<ListNode>>) -> bool {
        let mut slow = head.as_ref();
        let mut fast = head.as_ref();
        while let (Some(s), Some(f)) = (slow, fast.and_then(|node| node.next.as_ref())) {
            if std::ptr::eq(s, f) {
                return true;
            }
            slow = s.next.as_ref();
            fast = f.next.as_ref();
        }
        false
    }
    //  7.5环形链表 II【中等】
    /// 给定一个链表，返回链表开始入环的第一个节点。
    /// 如果链表无环，则返回 null。
    pub fn detect_cycle(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut slow = head.as_ref();
        let mut fast = head.as_ref();
        while let (Some(s), Some(f)) = (slow, fast.and_then(|node| node.next.as_ref())) {
            if std::ptr::eq(s, f) {
                let mut entry = head.as_ref();
                while !std::ptr::eq(s, entry.unwrap()) {
                    entry = entry.unwrap().next.as_ref();
                    slow = slow.unwrap().next.as_ref();
                }
                return Some(Box::new(ListNode { val: s.val, next: None }));
            }
            slow = s.next.as_ref();
            fast = f.next.as_ref();
        }
        None
    }
    //  7.6合并两个有序链表【简单】
    /// 将两个升序链表合并为一个新的 升序 链表并返回。新链表是通过拼接给定的两个链表的所有节点组成的。
    pub fn merge_two_lists(list1: Option<Box<ListNode>>, list2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut dummy = Box::new(ListNode { val: 0, next: None });
        let mut tail = &mut dummy;
        let (mut l1, mut l2) = (list1, list2);
        while l1.is_some() && l2.is_some() {
            if l1.as_ref().unwrap().val < l2.as_ref().unwrap().val {
                tail.next = l1;
                tail = tail.next.as_mut().unwrap();
                l1 = tail.next.take();
            } else {
                tail.next = l2;
                tail = tail.next.as_mut().unwrap();
                l2 = tail.next.take();
            }
        }
        tail.next = if l1.is_some() { l1 } else { l2 };
        dummy.next
    }
    //  7.7两数相加【中等】
    /// 给出两个 非空 的链表用来表示两个非负的整数。其中，它们各自的位数是按照逆序的方式存储的，并且它们的每个节点只能存储一位数字。
    /// 如果，我们将这两个数相加起来，则会返回一个新的链表来表示它们的和。您可以假设除了数字 0 之外，这两个数都不会以 0 开头。
    pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut dummy = Box::new(ListNode { val: 0, next: None });
        let mut tail = &mut dummy;
        let (mut p1, mut p2) = (l1, l2);
        let mut carry = 0;
        while p1.is_some() || p2.is_some() || carry != 0 {
            let sum = carry + p1.as_ref().map_or(0, |node| node.val) + p2.as_ref().map_or(0, |node| node.val);
            carry = sum / 10;
            tail.next = Some(Box::new(ListNode {
                val: sum % 10,
                next: None,
            }));
            tail = tail.next.as_mut().unwrap();
            if let Some(node) = p1 {
                p1 = node.next;
            }
            if let Some(node) = p2 {
                p2 = node.next;
            }
        }
        dummy.next
    }
    //  7.8删除链表的倒数第N个节点【中等】
    /// 给定一个链表，删除链表的倒数第 n 个节点，并返回链表的头节点。
    pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        let mut dummy = Box::new(ListNode { val: 0, next: head });
        //  先算长度（只读，不涉及 &mut）
        let mut len = 0;
        let mut p = &dummy.next;
        while let Some(node) = p {
            len += 1;
            p = &node.next;
        }
        //  找到要删除的前一个节点
        let mut p = &mut dummy;
        for _ in 0..(len - n as usize) {
            p = p.next.as_mut().unwrap();
        }
        //  删除节点
        let next = p.next.as_mut().unwrap().next.take();
        p.next = next;
        dummy.next
    }
    //  7.9两两交换链表中的节点【中等】
    /// 给定一个链表，两两交换其中相邻的节点，并返回交换后的链表。
    /// 你不能只是单纯的改变节点内部的值，而是需要实际的进行节点交换。
    pub fn swap_pairs(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut dummy = Box::new(ListNode { val: 0, next: head });
        let mut p = &mut dummy;
        while p.next.is_some() && p.next.as_ref().unwrap().next.is_some() {
            let mut first = p.next.take().unwrap();
            let mut second = first.next.take().unwrap();
            first.next = second.next.take();
            second.next = Some(first);
            p.next = Some(second);
            p = p.next.as_mut().unwrap().next.as_mut().unwrap();
        }
        dummy.next
    }
    //  7.10 K个一组翻转链表【困难】
    /// 给定一个链表，每 k 个节点一组进行翻转，并返回翻转后的链表。
    /// k 是一个正整数，它的值小于或等于链表的长度。
    /// 如果节点总数不是 k 的整数倍，那么将最后剩余的节点保持原有顺序。
    pub fn reverse_k_group(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        head
    }
    //  7.11 随机链表的复制【中等】
    /// 给你一个长度为 n 的链表，每个节点包含一个额外增加的随机指针 random，该指针可以指向链表中的任何节点或空节点。请你返回这个链表的 深拷贝 。  
    /// 深拷贝应该正好由 n 个 全新 的节点组成，其中每个新节点的值都设为其对应的原节点的值。新节点的 next 指针和 random 指针也应当指向对应原节点的 next 指针和 random 指针所指向的节点。
    /// 如果原节点的 random 指针指向 null ，则新节点的 random 指针也应当指向 null 。  
    /// 你必须返回对链表的 深拷贝 ，而不是对原链表的引用。
    pub fn copy_random_list(head: Option<Box<RandomListNode>>) -> Option<Box<RandomListNode>> {
        head
    }
    //  7.12排序链表【中等】
    /// 给你链表的头结点 head ，请将其按 升序排列并返回排序后的链表。
    pub fn sort_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        head
    }
    //  7.13合并K个排序链表【困难】
    /// 给你一个链表数组，每个链表都已经按升序排列。
    pub fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        None
    }
    //  7.14 LRU缓存【中等】
    /// 运用你所掌握的数据结构，设计和实现一个  LRU (最近最少使用) 缓存机制 。它应该支持以下操作： 获取数据 get 和 写入数据 put 。
    /// 获取数据 get(key) - 如果密钥 (key) 存在于缓存中，则获取密钥的值（总是正数），否则返回 -1。
    /// 写入数据 put(key, value) - 如果密钥已经存在，则变更其数据值；如果密钥不存在，则插入该组「密钥-值」。当缓存容量达到上限时，它应该在写入新数据之前删除最久未使用的数据值，从而为新的数据值留出空间。
    /// 请你实现 LRUCache 类：
    /// LRUCache(int capacity) 以正整数作为容量 capacity 初始化 LRU 缓存
    /// int get(int key) 如果密钥存在于缓存中，则返回密钥的值，否则返回 -1 。
    /// void put(int key, int value) 如果密钥已经存在，则变更其数据值；如果密钥不存在，则插入该组「密钥-值」。当缓存容量达到上限时，它应该在写入新数据之前删除最久未使用的数据值，从而为新的数据值留出空间。  
    /// 你必须在 O(1) 时间复杂度内完成 get 和 put 操作。

    //  8树
    //  8.1二叉树的中序遍历【简单】
    /// 给定一个二叉树的根节点 root ，返回它的 中序 遍历。
    pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut result = Vec::new();
        let mut stack = Vec::new();
        let mut current = root;
        while current.is_some() || !stack.is_empty() {
            while let Some(node) = current {
                stack.push(node.clone());
                current = node.borrow().left.clone();
            }
            if let Some(node) = stack.pop() {
                result.push(node.borrow().val);
                current = node.borrow().right.clone();
            }
        }
        result
    }
    //  8.2二叉树的最大深度【简单】
    /// 给定一个二叉树，找出其最大深度。
    /// 二叉树的深度为根节点到最远叶子节点的最长路径上的节点数。
    pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if let Some(node) = root {
            let left_depth = Self::max_depth(node.borrow().left.clone());
            let right_depth = Self::max_depth(node.borrow().right.clone());
            1 + left_depth.max(right_depth)
        } else {
            0
        }
    }
    //  8.3翻转二叉树【简单】
    /// 给你一棵二叉树的根节点 root ，翻转这棵二叉树，并返回其根节点。
    pub fn invert_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(node) = root {
            let left = Self::invert_tree(node.borrow().left.clone());
            let right = Self::invert_tree(node.borrow().right.clone());
            node.borrow_mut().left = right;
            node.borrow_mut().right = left;
            Some(node)
        } else {
            None
        }
    }
    //  8.4对称二叉树【简单】
    /// 给你一个二叉树的根节点 root ，检查它是否轴对称。
    pub fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        fn is_mirror(left: Option<Rc<RefCell<TreeNode>>>, right: Option<Rc<RefCell<TreeNode>>>) -> bool {
            match (left, right) {
                (None, None) => true,
                (Some(l), Some(r)) => {
                    l.borrow().val == r.borrow().val
                        && is_mirror(l.borrow().left.clone(), r.borrow().right.clone())
                        && is_mirror(l.borrow().right.clone(), r.borrow().left.clone())
                }
                _ => false,
            }
        }
        if let Some(node) = root {
            is_mirror(node.borrow().left.clone(), node.borrow().right.clone())
        } else {
            true
        }
    }
    //  8.5二叉树的直径【简单】
    /// 给定一棵二叉树，你需要计算它的直径长度。二叉树的直径长度是树中任意两个节点路径长度中的最大值。这条路径可能穿过也可能不穿过根节点。
    pub fn diameter_of_binary_tree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut diameter = 0;
        fn depth(node: Option<Rc<RefCell<TreeNode>>>, diameter: &mut i32) -> i32 {
            if let Some(n) = node {
                let left_depth = depth(n.borrow().left.clone(), diameter);
                let right_depth = depth(n.borrow().right.clone(), diameter);
                *diameter = (*diameter).max(left_depth + right_depth);
                1 + left_depth.max(right_depth)
            } else {
                0
            }
        }
        depth(root, &mut diameter);
        diameter
    }
    //  8.6二叉树的层序遍历【中等】
    /// 给你一个二叉树，请你返回其按 层序遍历 得到的节点值。 （即逐层地，从左到右访问所有节点）。
    pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut result = Vec::new();
        if root.is_none() {
            return result;
        }
        let mut queue = std::collections::VecDeque::new();
        queue.push_back(root.unwrap());
        while !queue.is_empty() {
            let level_size = queue.len();
            let mut level_values = Vec::with_capacity(level_size);
            for _ in 0..level_size {
                if let Some(node) = queue.pop_front() {
                    level_values.push(node.borrow().val);
                    if let Some(left) = node.borrow().left.clone() {
                        queue.push_back(left);
                    }
                    if let Some(right) = node.borrow().right.clone() {
                        queue.push_back(right);
                    }
                }
            }
            result.push(level_values);
        }
        result
    }
    //  8.7将有序数组转换为二叉搜索树【简单】
    /// 给你一个整数数组 nums ，其中元素已经按 升序 排列，请你将其转换为一棵高度平衡二叉搜索树。
    /// 本题中，一棵高度平衡二叉树是一棵满足每个节点的左右两个子树的高度差不超过 1 的二叉树。
    pub fn sorted_array_to_bst(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        fn build_bst(nums: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
            if nums.is_empty() {
                return None;
            }
            let mid = nums.len() / 2;
            let node = Rc::new(RefCell::new(TreeNode {
                val: nums[mid],
                left: build_bst(&nums[..mid]),
                right: build_bst(&nums[mid + 1..]),
            }));
            Some(node)
        }
        build_bst(&nums)
    }
    //  8.8验证二叉搜索树【中等】
    /// 给定一个二叉树，判断其是否是一个有效的二叉搜索树。
    pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        fn validate(node: Option<Rc<RefCell<TreeNode>>>, min: Option<i32>, max: Option<i32>) -> bool {
            match node {
                None => true,
                Some(n) => {
                    let val = n.borrow().val;
                    if let Some(min_val) = min {
                        if val <= min_val {
                            return false;
                        }
                    }
                    if let Some(max_val) = max {
                        if val >= max_val {
                            return false;
                        }
                    }
                    validate(n.borrow().left.clone(), min, Some(val))
                        && validate(n.borrow().right.clone(), Some(val), max)
                }
            }
        }
        validate(root, None, None)
    }
    //  8.9二叉搜索树中第K小的元素【中等】
    /// 给定一个二叉搜索树，编写一个函数 kthSmallest 来查找其中第 k 个最小的元素。
    pub fn kth_smallest(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> i32 {
        let mut stack = Vec::new();
        let mut current = root;
        let mut count = 0;
        while current.is_some() || !stack.is_empty() {
            while let Some(node) = current {
                stack.push(node.clone());
                current = node.borrow().left.clone();
            }
            if let Some(node) = stack.pop() {
                count += 1;
                if count == k {
                    return node.borrow().val;
                }
                current = node.borrow().right.clone();
            }
        }
        -1 // 如果 k 超出范围，返回 -1
    }
    //  8.10二叉树的右视图【中等】
    /// 给定一个二叉树的根节点 root，想象自己站在它的右侧，返回从右侧所能看到的节点值。
    pub fn right_side_view(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut result = Vec::new();
        if root.is_none() {
            return result;
        }
        let mut queue = std::collections::VecDeque::new();
        queue.push_back(root.unwrap());
        while !queue.is_empty() {
            let level_size = queue.len();
            for i in 0..level_size {
                if let Some(node) = queue.pop_front() {
                    if i == level_size - 1 {
                        result.push(node.borrow().val);
                    }
                    if let Some(left) = node.borrow().left.clone() {
                        queue.push_back(left);
                    }
                    if let Some(right) = node.borrow().right.clone() {
                        queue.push_back(right);
                    }
                }
            }
        }
        result
    }
    //  8.11二叉树展开为链表【中等】
    /// 给定一个二叉树，原地 将它展开为一个单链表。
    /// 展开后的单链表应该使用二叉树原有的右子指针，左子指针应该全部置为 null 。
    pub fn flatten(root: &mut Option<Rc<RefCell<TreeNode>>>) {
        if let Some(node) = root {
            Self::flatten(&mut node.borrow_mut().left);
            Self::flatten(&mut node.borrow_mut().right);
            let left = node.borrow_mut().left.take();
            let right = node.borrow_mut().right.take();
            node.borrow_mut().right = left;
            let mut current = node.clone();
            loop {
                let next = current.borrow().right.clone();
                if let Some(next_node) = next {
                    current = next_node;
                } else {
                    break;
                }
            }
            current.borrow_mut().right = right;
        }
    }
    //  8.12从前序和中序遍历构造二叉树【中等】
    /// 给定两个整数数组 preorder 和 inorder ，其中 preorder 是二叉树的先序遍历，inorder 是同一棵树的中序遍历，请你构造并返回这颗二叉树。
    pub fn build_tree(preorder: Vec<i32>, inorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        fn build(preorder: &[i32], inorder: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
            if preorder.is_empty() || inorder.is_empty() {
                return None;
            }
            let root_val = preorder[0];
            let root = Rc::new(RefCell::new(TreeNode {
                val: root_val,
                left: None,
                right: None,
            }));
            if let Some(root_index) = inorder.iter().position(|&x| x == root_val) {
                root.borrow_mut().left = build(&preorder[1..=root_index], &inorder[..root_index]);
                root.borrow_mut().right = build(&preorder[root_index + 1..], &inorder[root_index + 1..]);
            }
            Some(root)
        }
        build(&preorder, &inorder)
    }
    //  8.13路径总和 III【中等】
    /// 给定一个二叉树的根节点 root 和一个整数 targetSum ，求该二叉树里节点值之和等于 targetSum 的路径的数目。
    /// 路径 不需要从根节点开始，也不需要在叶子节点结束，但路径方向必须向下（只能从父节点到子节点）。
    pub fn path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> i32 {
        fn dfs(node: Option<Rc<RefCell<TreeNode>>>, target_sum: i32, current_sum: i32, count: &mut i32) {
            if let Some(n) = node {
                let new_sum = current_sum + n.borrow().val;
                if new_sum == target_sum {
                    *count += 1;
                }
                dfs(n.borrow().left.clone(), target_sum, new_sum, count);
                dfs(n.borrow().right.clone(), target_sum, new_sum, count);
            }
        }
        let mut count = 0;
        dfs(root, target_sum, 0, &mut count);
        count
    }
    //  8.14二叉树的最近公共祖先【中等】
    /// 给定一个二叉树, 找到该树中两个指定节点的最近公共祖先。
    /// 根据维基百科中对最近公共祖先的定义：“对于有根树
    /// T 的两个结点 p、q，最近公共祖先表示为一个结点 x，满足 x 是 p、q 的祖先且 x 的深度尽可能大（一个节点也可以是它自己的祖先）。”
    pub fn lowest_common_ancestor(
        root: Option<Rc<RefCell<TreeNode>>>,
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(node) = root {
            let condition1 = Rc::as_ptr(p.as_ref().unwrap());
            let condition2 = Rc::as_ptr(q.as_ref().unwrap());
            if std::ptr::eq(node.as_ref(), condition1) || std::ptr::eq(node.as_ref(), condition2) {
                return Some(node);
            }
            let left = Self::lowest_common_ancestor(node.borrow().left.clone(), p.clone(), q.clone());
            let right = Self::lowest_common_ancestor(node.borrow().right.clone(), p.clone(), q.clone());
            if left.is_some() && right.is_some() {
                return Some(node);
            }
            left.or(right)
        } else {
            None
        }
    }
    //  8.15二叉树的最大路径和【困难】
    /// 给定一个非空二叉树，返回其最大路径和。
    /// 路径被定义为一条从树中任意节点出发，沿父节点-子节点连接，达到任意节点的序列。该路径至少包含一个节点，且不一定经过根节点。
    pub fn max_path_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut max_sum = i32::MIN;
        fn dfs(node: Option<Rc<RefCell<TreeNode>>>, max_sum: &mut i32) -> i32 {
            if let Some(n) = node {
                let left_sum = dfs(n.borrow().left.clone(), max_sum).max(0);
                let right_sum = dfs(n.borrow().right.clone(), max_sum).max(0);
                let current_sum = n.borrow().val + left_sum + right_sum;
                *max_sum = (*max_sum).max(current_sum);
                n.borrow().val + left_sum.max(right_sum)
            } else {
                0
            }
        }
        dfs(root, &mut max_sum);
        max_sum
    }

    fn other() {}
}

// pub struct LRUCache {
//     capacity: usize,
//     map: std::collections::HashMap<i32, (i32, std::collections::LinkedList<i32>::CursorMut)>,
//     usage: std::collections::LinkedList<i32>,
// }
