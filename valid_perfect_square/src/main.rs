fn main() {
    assert_eq!(Solution::is_perfect_square(16), true);
    assert_eq!(Solution::is_perfect_square(14), false);
    assert_eq!(Solution::is_perfect_square(1), true);
    assert_eq!(Solution::is_perfect_square(2000105819), false);
}

struct Solution;

// my first instinct was to create a vec out of (0..=num), but that was dumb because it takes way too long to just do that if num is big
// the highlight here is that checked_pow really matters
impl Solution {
    pub fn is_perfect_square(num: i32) -> bool {
        let mut l = 1;
        let mut r = num;
        while l < r {
            let m = (r + l) / 2;
            match m.checked_pow(2) {
                Some(square) if num > square => l = m + 1,
                _ => r = m,
            }
        }
        r.pow(2) == num
    }
}
