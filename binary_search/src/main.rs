fn main() {
    assert_eq!(Solution::search(vec![-1, 0, 3, 5, 9, 12], 9), 4);
    assert_eq!(Solution::search(vec![-1, 0, 3, 5, 9, 12], 2), -1);
    assert_eq!(Solution::search(vec![5], 5), 0);
}

struct Solution {}
impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let mut l = 0;
        let mut r = nums.len() - 1;

        if target < nums[l] || target > nums[r] {
            return -1;
        }

        while l < r {
            // I'm going to continuously reduce the possible range
            // m rounds down, so it may end up being l
            // as long as l < r, which is the condition of the while loop, m < r
            let m = (l + r) / 2;
            // if target is less than or equal to nums[m], that means we can safely set r to m, and r will become smaller, meaning our range has reduced
            // otherwise, the target is greater than nums[m], so it is definitely sensible to set l to m + 1, which will also reduce our range, but may run the risk of sending l beyond the target's index
            if target <= nums[m] {
                r = m;
            } else {
                l = m + 1;
            }
        }

        // since l has a risk of exceeding the target index but r does not r is guaranteed to be our target index
        if nums[r] == target {
            r as i32
        } else {
            -1
        }
    }
}
