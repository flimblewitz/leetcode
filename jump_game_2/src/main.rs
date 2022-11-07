fn main() {
    println!("{:?}", Solution::jump(vec![2, 3, 1, 1, 4]));
}

struct Solution {}
impl Solution {
    pub fn jump(nums: Vec<i32>) -> i32 {
        let mut jumps = vec![std::i32::MAX; nums.len()];
        jumps[0] = 0;
        for i in 0..jumps.len() {
            // this is awkward and I could make it better, but whatever
            for j in 0..=(nums[i] as usize) {
                if i + j >= jumps.len() {
                    break;
                }
                jumps[i + j] = jumps[i + j].min(1 + jumps[i]);
            }
        }

        jumps[jumps.len() - 1]
    }
}
