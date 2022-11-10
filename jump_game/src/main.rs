fn main() {
    println!("{:?}", Solution::can_jump(vec![2, 3, 1, 1, 4]));
    println!("{:?}", Solution::can_jump(vec![3, 2, 1, 0, 4]));
}

struct Solution {}
impl Solution {
    pub fn can_jump(nums: Vec<i32>) -> bool {
        let mut i = 0;
        while i < nums.len() {
            let jump_power = nums[i] as usize;
            let furthest_accessible_index = i + jump_power;
            if furthest_accessible_index >= nums.len() - 1 {
                return true;
            }
            if let Some((_iterator_index, index_of_next_best_jump)) =
                (i..=furthest_accessible_index).enumerate().max_by_key(
                    |(iterator_index, nums_index)| *iterator_index + nums[*nums_index] as usize,
                )
            {
                if index_of_next_best_jump == i {
                    break;
                }
                i = index_of_next_best_jump;
                continue;
            }
            break;
        }
        false
    }
}
