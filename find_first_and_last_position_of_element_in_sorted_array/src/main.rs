fn main() {
    assert_eq!(
        Solution::search_range(vec![5, 7, 7, 8, 8, 10], 8),
        vec![3, 4]
    );
    assert_eq!(
        Solution::search_range(vec![5, 7, 7, 8, 8, 10], 6),
        vec![-1, -1]
    );
    assert_eq!(Solution::search_range(vec![], 0), vec![-1, -1]);
    assert_eq!(Solution::search_range(vec![2, 2], 2), vec![0, 1]);
}

struct Solution;

impl Solution {
    pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
        if nums.is_empty() {
            return vec![-1, -1];
        }

        // let's find the first occurrence
        let mut l = 0;
        let mut r = nums.len() - 1;
        while l < r {
            // println!("{left}, {right}");
            let m = (r + l) / 2;
            if target <= nums[m] {
                r = m;
            } else {
                l = m + 1;
            }
        }
        let first = if nums[r] == target { r as i32 } else { -1 };

        // and now for the last occurrence
        let mut l = 0;
        let mut r = nums.len() - 1;
        while l < r {
            // println!("{left}, {right}");
            let m = (r + l) / 2;
            // let's try setting r to the first index with a value greater than the target
            if target >= nums[m] {
                l = m + 1;
            } else {
                r = m;
            }
        }
        // so we know that r is either the last index (and the last occurrence of the target), or it's the smallest number that's still larger than the target
        // if it's larger than the target and there's at least one preceding index, let's bump it down one because that might be our answer
        if target < nums[r] && r > 0 {
            r -= 1;
        }
        let last = if nums[r] == target { r as i32 } else { -1 };

        vec![first, last]
    }
}
