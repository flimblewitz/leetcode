fn main() {
    // println!("{:?}", Solution::solve_n_queens(2));
    println!(
        "{:?}",
        Solution::max_sub_array(vec![-2, 1, -3, 4, -1, 2, 1, -5, 4])
    );
    println!("{:?}", Solution::max_sub_array(vec![5, 4, -1, 7, 8]));
}

struct Solution {}
impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let mut biggest_sum = nums[0];
        let mut i = 0;
        while i < nums.len() {
            let mut acc = 0;
            for j in i..(nums.len()) {
                acc += nums[j];
                biggest_sum = biggest_sum.max(acc);
                // we're done considering subarrays starting at i as soon as we either hit a sum of 0 or below (it can't help any other inclusive trailing subarray's sum) OR if we've hit the end of the array. In either case, it's time to set the next i to one beyond where we are now
                if acc <= 0 || j == nums.len() - 1 {
                    i = j + 1;
                    break;
                }
            }
        }
        biggest_sum
    }
}
