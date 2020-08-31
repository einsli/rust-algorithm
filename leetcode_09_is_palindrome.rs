/*
T9
判断一个整数是否是回文数。回文数是指正序（从左向右）和倒序（从右向左）读都是一样的整数。

示例 1:

输入: 121
输出: true
示例 2:

输入: -121
输出: false
解释: 从左向右读, 为 -121 。 从右向左读, 为 121- 。因此它不是一个回文数。
示例 3:

输入: 10
输出: false
解释: 从右向左读, 为 01 。因此它不是一个回文数。
进阶:

你能不将整数转为字符串来解决这个问题吗？

来源：力扣（LeetCode）
链接：https://leetcode-cn.com/problems/palindrome-number
著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
*/


struct Solution {}

impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        if x >= 0 && x <= 10 {
            return false
        }
        if x < 0 || x % 10 == 0 {
            return false
        }

        let mut x1: i32 = x;

        let mut reverse: i32 = 0;

        loop {
            if x1 == 0 {
                break
            }

            reverse = reverse * 10 + x1 % 10;
            x1 = x1 / 10;
        }

        let res = reverse.clone();

        if x == res || x == res / 10 {
            return true
        }

        // 到这里是最后返回了，可以不用return
        false
    }
}


fn main() {
    let target = 11;
    let reverse_res = Solution::is_palindrome(target);
    println!("res {}", reverse_res);
}
