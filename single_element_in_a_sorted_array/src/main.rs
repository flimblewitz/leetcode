fn main() {
    assert_eq!(Solution::single_non_duplicate(vec![1, 1, 2, 3, 3]), 2);
    assert_eq!(
        Solution::single_non_duplicate(vec![1, 1, 2, 3, 3, 4, 4, 8, 8]),
        2
    );
    assert_eq!(
        Solution::single_non_duplicate(vec![3, 3, 7, 7, 10, 11, 11]),
        10
    );
    assert_eq!(Solution::single_non_duplicate(vec![1, 1, 2, 2, 3]), 3);
    assert_eq!(Solution::single_non_duplicate(vec![1, 2, 2, 3, 3]), 1);
    assert_eq!(
        Solution::single_non_duplicate(vec![1, 1, 2, 2, 4, 4, 5, 5, 9]),
        9
    );
}
struct Solution;

impl Solution {
    pub fn single_non_duplicate(nums: Vec<i32>) -> i32 {
        let mut l = 0;
        let mut r = nums.len() - 1;
        // the premise of the problem (an odd array with one singleton and only n>=0 pairs of duplicates otherwise)
        // we'll maintain this with each iteration
        while l < r {
            let m = (l + r) / 2;
            if m > 0 && nums[m - 1] == nums[m] {
                // given that m has a duplicate to the left, if m-l is even, then that means the left of m has some number of pairs of duplicates to the left, _and_ m's duplicate, _and_ the singleton
                // and if the singleton is on the left, then we can safely set r = m - 2 because there must be at least two things to the left of m, and we want to skip past m's duplicate
                // else, the singleton must be on the right, in which case we can safely set l = m + 1 because there must be at least one thing to the right, and we don't want to bother considering m or its duplicate in the next iteration
                if (m - l) % 2 == 0 {
                    r = m - 2
                } else {
                    l = m + 1
                }
            } else if nums[m + 1] == nums[m] {
                // given that m has a duplicate to the right, if m-l is even, then there are only pairs of duplicates to the left, meaning that the singleton must be on the right
                // and if the singleton is on the right, then we can safely set l = m + 2 because there must be at least two things to the right of m, and we want to skip past m's duplicate
                // else, the singleton must be on the left, in which case we can safely set r = m - 1 because there must be at least one thing to the left, and we don't want to bother considering m or its duplicate in the next iteration
                if (m - l) % 2 == 0 {
                    l = m + 2
                } else {
                    r = m - 1
                }
            } else {
                return nums[m];
            }
        }
        nums[r]
    }
}
