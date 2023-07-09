fn main() {
    assert_eq!(Solution::search_insert(vec![1, 3, 5, 6], 5), 2);
    assert_eq!(Solution::search_insert(vec![1, 3, 5, 6], 2), 1);
    assert_eq!(Solution::search_insert(vec![1, 3, 5, 6], 7), 4);
}

struct Solution {}
impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        let mut l = 0;
        let mut r = nums.len() - 1;

        while l < r {
            let m = (l + r) / 2;
            // println!("m: {m}");
            if target <= nums[m] {
                r = m;
                // println!("r: {r}")
            } else {
                l = m + 1;
                // println!("l: {l}")
            }
        }

        // so we know that r is the index of the target if it's in the array
        // but if it's not in the array, it either has to become the new resident of r if the target is less than nums[r] or it has to become the resident of r+1
        if target <= nums[r] {
            r as i32
        } else {
            (r + 1) as i32
        }
    }
}
