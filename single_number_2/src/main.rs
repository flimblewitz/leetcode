fn main() {
    println!("{:?}", Solution::single_number(vec![2, 2, 3, 2]));
}

struct Solution {}
impl Solution {
    // sorting is POWERFUL. Remember that.
    pub fn single_number(mut nums: Vec<i32>) -> i32 {
        if nums.len() == 1 {
            return nums[0];
        }
        nums.sort_unstable();
        nums.iter()
            .enumerate()
            .find(|(i, num)| {
                let l = nums.len() - 1;
                match i {
                    0 => nums[i + 1] != **num,
                    ll if *ll == l => nums[i - 1] != **num,
                    _ => nums[i + 1] != **num && nums[i - 1] != **num,
                }
            })
            .unwrap()
            .1
            .to_owned()
    }
}
