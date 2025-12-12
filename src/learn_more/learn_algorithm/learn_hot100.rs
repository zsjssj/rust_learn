//! leetcode热题100算法题解答

#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
use super::Solution;

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
    // 2.2盛最多水的容器【中等】
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
    // 2.3三数之和【中等】
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
    // 2.4接雨水【困难】
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

    // 3.滑动窗口
    // 3.1无重复字符的最长子串【中等】
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

    // 4.子串
    // 4.1和为K的子数组【中等】
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
    // 4.2窗口滑动中的最大值【困难】
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
    // 4.3最小覆盖子串【困难】
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

    // 5.普通数组
    // 5.1 最大子数组和【中等】
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
    // 5.2合并区间【中等】
    ///以数组 intervals 表示若干个区间的集合，其中单个区间为 intervals[i] = [starti, endi] 。
    ///请你合并所有重叠的区间，并返回一个不重叠的区间数组，该数组需恰好覆盖输入中的所有区间。
    pub fn merge(intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut intervals = intervals;
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
}
