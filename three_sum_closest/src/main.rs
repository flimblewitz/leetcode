fn main() {
    // println!("{:?}", Solution::three_sum_closest(vec![-1, 2, 1, -4], 1));
    println!(
        "{:?}",
        Solution::three_sum_closest(vec![4, 0, 5, -5, 3, 3, 0, -4, -5], -2)
    );
}

struct Solution {}
impl Solution {
    pub fn three_sum_closest(nums: Vec<i32>, target: i32) -> i32 {
        let mut nums = nums.clone();
        nums.sort();
        let mut closest_sum = nums[0] + nums[1] + nums[2];
        let mut closest_sum_diff = (target - closest_sum).abs();
        println!(
            "target: {target}, closest_sum: {closest_sum}, closest_sum_diff: {closest_sum_diff}"
        );
        // let mut smallest_diff = ();
        let mut closest_indices = (0, 1, 2);
        for i in 0..(nums.len() - 2) {
            for j in (i + 1)..(nums.len() - 1) {
                for k in (j + 1)..nums.len() {
                    let sum = nums[i] + nums[j] + nums[k];
                    let sum_diff = (target - sum).abs();
                    println!("{i}, {j}, {k}, sum: {sum}, sum_diff: {sum_diff}");
                    if sum_diff < closest_sum_diff {
                        closest_indices = (i, j, k);
                        closest_sum = sum;
                        closest_sum_diff = sum_diff;
                    }
                }
            }
        }
        closest_sum
    }
}
