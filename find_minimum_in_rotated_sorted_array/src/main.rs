fn main() {
    assert_eq!(Solution::find_min(vec![3, 4, 5, 1, 2]), 1);
}

struct Solution;

// note that the nums are unique
// I know that at least one rotation has happened
// so if nums[m] < nums[r], then the minimum is at or before m, so it's safe to set r to m
// otherwise, the min must be after m, so set l to m + 1

impl Solution {
    pub fn find_min(nums: Vec<i32>) -> i32 {
        let mut l = 0;
        let mut r = nums.len() - 1;
        while l < r {
            let m = (l + r) / 2;
            if nums[m] < nums[r] {
                r = m;
            } else {
                l = m + 1;
            }
        }
        nums[r]
    }
}
