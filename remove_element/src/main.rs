fn main() {
    let mut v = vec![3, 2, 2, 3];
    println!("{}, {:?}", Solution::remove_element(&mut v, 3), v);
    let mut v = vec![0, 1, 2, 2, 3, 0, 4, 2];
    println!("{}, {:?}", Solution::remove_element(&mut v, 2), v);
}

struct Solution {}
impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        *nums = nums.iter().filter(|num| **num != val).cloned().collect();
        nums.len() as i32
    }
}
