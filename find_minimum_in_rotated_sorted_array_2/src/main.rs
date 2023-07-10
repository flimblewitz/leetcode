fn main() {
    assert_eq!(Solution::find_min(vec![1, 3, 5]), 1);
    assert_eq!(Solution::find_min(vec![2, 2, 2, 0, 1]), 0);
    assert_eq!(Solution::find_min(vec![3, 3, 1, 3]), 1);
    assert_eq!(Solution::find_min(vec![1, 3, 3]), 1);
    assert_eq!(Solution::find_min(vec![10, 1, 10, 10, 10]), 1);
    assert_eq!(Solution::find_min(vec![10, 10, 10, 1, 10]), 1);
}

struct Solution;

// note that the nums may be duplicated
// nums may not be rotated
// if nums[m] < nums[r], then the minimum is at or before m, so it's safe to set r to m
// if nums[m] > nums[r], then the min must be after m, so set l to m + 1
// if nums[m] == nums[r], we have no idea what the hell is going on. In this case, we can just set r to r-1 because the min is still somewhere in before r (it doesn't matter if nums[r] is a duplicate of the min)

impl Solution {
    pub fn find_min(nums: Vec<i32>) -> i32 {
        let mut l = 0;
        let mut r = nums.len() - 1;
        while l < r {
            let m = (l + r) / 2;
            if nums[m] < nums[r] {
                r = m;
            } else if nums[m] > nums[r] {
                l = m + 1;
            } else {
                r = r - 1;
            }
        }
        nums[r]
    }
}
