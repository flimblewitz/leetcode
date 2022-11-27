fn main() {
    println!("{:?}", Solution::single_number(vec![1, 2, 1, 3, 2, 5]));
}

struct Solution {}
impl Solution {
    // sorting is POWERFUL. Remember that.
    pub fn single_number(mut nums: Vec<i32>) -> Vec<i32> {
        nums.sort_unstable();
        nums.iter()
            .enumerate()
            .filter(|(i, num)| {
                let l = nums.len() - 1;
                match i {
                    0 => nums[i + 1] != **num,
                    ll if *ll == l => nums[i - 1] != **num,
                    _ => nums[i + 1] != **num && nums[i - 1] != **num,
                }
            })
            .map(|(_, num)| *num)
            .collect()
    }
}
