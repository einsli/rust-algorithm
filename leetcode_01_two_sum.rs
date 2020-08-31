/*
T1

给定一个整数数组 nums 和一个目标值 target，请你在该数组中找出和为目标值的那 两个 整数，并返回他们的数组下标。

你可以假设每种输入只会对应一个答案。但是，数组中同一个元素不能使用两遍。

 

示例:

给定 nums = [2, 7, 11, 15], target = 9

因为 nums[0] + nums[1] = 2 + 7 = 9

来源：力扣（LeetCode）
链接：https://leetcode-cn.com/problems/two-sum
著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
*/
use std::collections::HashMap;

struct Solution{}

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map = HashMap::with_capacity(nums.len());
        for (index, &num) in nums.iter().enumerate() {
            match map.get(&(target-num)){
                None => {map.insert(num, index);},
                Some(sub_index) => {return vec![*sub_index as i32, index as i32]},
            }
        }
        vec![]
    }
}

fn main() {
    // let sol = Solution{};
    let nums: Vec<i32> = vec![2, 7, 11, 15];
    let target:i32 = 9;
    let res = Solution::two_sum(nums, target);
    println!("the index is {:?}", res);
}
