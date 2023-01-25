fn main() {
    println!("{:?}", Solution::search_range(vec![5, 7, 8, 8, 10], 8));
    println!("{:?}", Solution::search_range(vec![5, 7, 7, 8, 8, 10], 8));
    println!("{:?}", Solution::search_range(vec![5, 7, 7, 8, 8, 10], 7));
    println!("{:?}", Solution::search_range(vec![5, 7, 7, 8, 8, 10], 6));
    println!("{:?}", Solution::search_range(vec![], 0));
    println!("{:?}", Solution::search_range(vec![2, 2], 2));
}

struct Solution;

impl Solution {
    pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
        if nums.is_empty() {
            return vec![-1, -1];
        }

        let mut left = 0;
        let mut right = nums.len() - 1;
        while left < right {
            // println!("{left}, {right}");
            let middle = (right + left) / 2;
            if nums[middle] >= target {
                right = middle;
            } else {
                left = middle + 1;
            }
        }
        let first = if nums[right] == target {
            right as i32
        } else {
            -1
        };

        let mut left = 0;
        let mut right = nums.len() - 1;
        while left < right {
            // println!("{left}, {right}");
            let middle = (right + left) / 2;
            if nums[middle] > target {
                right = middle;
            } else {
                if middle + 1 < nums.len() && nums[middle + 1] <= target {
                    left = middle + 1
                } else if left == middle {
                    right = middle;
                } else {
                    left = middle;
                }
            }
        }
        let last = if nums[right] == target {
            right as i32
        } else {
            -1
        };

        vec![first, last]
    }
}
