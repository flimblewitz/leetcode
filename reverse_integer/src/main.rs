fn main() {
    println!("{}", Solution::reverse(-123));
}

struct Solution {}
impl Solution {
    pub fn reverse(x: i32) -> i32 {
        let is_negative = x < 0;
        let abs_x = x.abs();

        let s = abs_x.to_string();

        let abs_reverse = s.chars().rev().collect::<String>().parse().unwrap_or(0);

        if is_negative {
            -1 * abs_reverse
        } else {
            abs_reverse
        }
    }
}
