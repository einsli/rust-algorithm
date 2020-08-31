
/*
T350

给定两个数组，编写一个函数来计算它们的交集。

示例 1：

输入：nums1 = [1,2,2,1], nums2 = [2,2]
输出：[2,2]
示例 2:

输入：nums1 = [4,9,5], nums2 = [9,4,9,8,4]
输出：[4,9]
 

说明：

输出结果中每个元素出现的次数，应与元素在两个数组中出现次数的最小值一致。
我们可以不考虑输出结果的顺序。
进阶：

如果给定的数组已经排好序呢？你将如何优化你的算法？
如果 nums1 的大小比 nums2 小很多，哪种方法更优？
如果 nums2 的元素存储在磁盘上，内存是有限的，并且你不能一次加载所有的元素到内存中，你该怎么办？

来源：力扣（LeetCode）
链接：https://leetcode-cn.com/problems/intersection-of-two-arrays-ii
著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。

*/


use std::collections::HashMap;

struct Solution{}

impl Solution {
    pub fn intersect(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let mut res = Vec::new();
        let nums_len = || {
            if nums1.len() < nums2.len() {
                nums1.len()
            }else {
                nums2.len()
            }
        };
        let mut map = HashMap::with_capacity(nums_len());

        for num in nums1 {
            let count = map.entry(num).or_insert(0);
            *count += 1;
        }

        for num in nums2 {
            match map.get(&num) {
                None => {},
                Some(_value) => {
                    let count = map.entry(num).or_insert(0);
                    if *count > 0 {
                        *count -= 1;
                        res.push(num)
                    }
                }
            }
        }
        res
    }
}

fn main() {
    let nums1: Vec<i32> = vec![1,4,4,9,5,1,1];
    let nums2: Vec<i32> = vec![9,4,9,8,4,1];
    let res = Solution::intersect(nums1, nums2);
    println!("the set is {:#?}", res);
}