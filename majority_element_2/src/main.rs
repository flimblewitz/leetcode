fn main() {
    println!("{:?}", Solution::majority_element(vec![3, 2, 3]));
    println!("{:?}", Solution::majority_element(vec![1]));
    println!("{:?}", Solution::majority_element(vec![1, 2]));
}

struct Solution;

use std::collections::HashMap;
impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> Vec<i32> {
        let mut element_occurrences: HashMap<i32, usize> = HashMap::new();
        let threshold = nums.len() / 3;
        for num in nums {
            element_occurrences
                .entry(num)
                .and_modify(|occurrences| *occurrences += 1)
                .or_insert(1);
        }
        element_occurrences
            .into_iter()
            .filter(|(_, occurrences)| *occurrences > threshold)
            .map(|(element, _)| element)
            .collect()
    }
}
