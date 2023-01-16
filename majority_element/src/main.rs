fn main() {
    println!("{}", Solution::majority_element(vec![3, 2, 3]));
    println!("{}", Solution::majority_element(vec![2, 2, 1, 1, 1, 2, 2]));
}

struct Solution;

use std::collections::HashMap;
impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        let mut element_occurrences: HashMap<i32, usize> = HashMap::new();
        let threshold = nums.len() / 2;
        for num in nums {
            let occurrences = element_occurrences
                .entry(num)
                .and_modify(|occurrences| *occurrences += 1)
                .or_insert(1);
            if *occurrences > threshold {
                return num;
            }
        }
        panic!("oops")
    }
}
