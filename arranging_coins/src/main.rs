fn main() {
    assert_eq!(Solution::arrange_coins(5), 2);
    assert_eq!(Solution::arrange_coins(1), 1);
    assert_eq!(Solution::arrange_coins(3), 2);
}

struct Solution;

impl Solution {
    pub fn arrange_coins(n: i32) -> i32 {
        let mut leftover_coins = n;
        let mut row_length = 1;
        while leftover_coins >= row_length {
            leftover_coins -= row_length;
            row_length += 1;
        }
        row_length - 1
    }
}
