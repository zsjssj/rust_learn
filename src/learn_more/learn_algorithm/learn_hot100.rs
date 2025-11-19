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
    ///给你一个字符串数组，请你将 字母异位词 组合在一起。可以按任意顺序返回结果列表。
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
    //  1.3最长连续序列【中等】
    /// 给定一个未排序的整数数组 nums ，找出数字连续的最长序列（不要求序列元素在原数组中连续）的长度。
    ///
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
        let mut char_index_map = HashMap::new();
        let (mut left, mut max_length) = (0, 0);
        for (right, c) in s.chars().enumerate() {
            if let Some(&prev_index) = char_index_map.get(&c) {
                if prev_index >= left {
                    left = prev_index + 1;
                }
            }
            char_index_map.insert(c, right);
            max_length = max_length.max(right - left + 1);
        }
        max_length as i32
    }
}
