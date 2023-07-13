fn main() {
    assert_eq!(Solution::h_index(vec![0, 4, 5, 5, 5]), 4);
    assert_eq!(Solution::h_index(vec![0, 1, 3, 5, 6]), 3);
    assert_eq!(Solution::h_index(vec![1, 2, 100]), 2);
    assert_eq!(Solution::h_index(vec![100]), 1);
    assert_eq!(Solution::h_index(vec![0, 1]), 1);
    assert_eq!(Solution::h_index(vec![1, 1, 2, 2]), 2);
    assert_eq!(Solution::h_index(vec![0, 0, 4, 4]), 2);
}
struct Solution;

impl Solution {
    pub fn h_index(citations: Vec<i32>) -> i32 {
        // h index is the max value of h such that at least the h papers have EACH been cited at least h times
        let mut l = 0;
        let mut r = citations.len() - 1;
        // this is unusual because we're looking for an unknown target, but we do know how to reduce the range based on what we're seeing for each iteration
        // we know that if the mth item's value is at least the number of items including and after it, nothing after it can possibly be any better, so we can set r = m and iterate again
        // in contrast, if the mth item's value is less than the number of items including and after it, nothing before it can possibly be any better, so we can set l = m + 1 and iterate again
        while l < r {
            let m = (l + r) / 2;
            if citations[m] >= (citations.len() - m) as i32 {
                r = m;
            } else {
                l = m + 1;
            }
        }
        citations[r].min((citations.len() - r) as i32)
    }
}
