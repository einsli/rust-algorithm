/*
1
给出一个 32 位的有符号整数，你需要将这个整数中每位上的数字进行反转。

示例 1:

输入: 123
输出: 321
 示例 2:

输入: -123
输出: -321
示例 3:

输入: 120
输出: 21
注意:

假设我们的环境只能存储得下 32 位的有符号整数，则其数值范围为 [−231,  231 − 1]。请根据这个假设，如果反转后整数溢出那么就返回 0。

来源：力扣（LeetCode）
链接：https://leetcode-cn.com/problems/reverse-integer
著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
*/

use std::i32::MAX;
use std::i32::MIN;

const MAX_D10: i32 = MAX / 10;
const MAX_I32: i64 = MAX as i64;
const MIN_I32: i64 = MIN as i64;

struct Solution {}

impl Solution {
    pub fn reverse(x: i32) -> i32 {
        let n_minus = x < 0;
        let mut x_abs = x.abs();
        let mut res = 0;
        while x_abs > 0 {
            if res > MAX_D10 {
                let res_long = res as i64 * 10 + x_abs as i64 % 10;

                if n_minus && res_long < MIN_I32 {
                    return 0
                }
                if res_long > MAX_I32 {
                    return 0
                }
                res = res_long as i32;
                break;
            }
            res = res * 10 + x_abs % 10;
            x_abs = x_abs / 10;
        }
        if n_minus {0-res} else {res}
    }
}

fn main() {
    let number: i32 = -45673;
    println!("origin number is {}", number);
    let reversed_number = Solution::reverse(number);
    println!("reversed number = {}", reversed_number);
}