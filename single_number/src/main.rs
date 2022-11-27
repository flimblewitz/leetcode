fn main() {
    println!("{:?}", Solution::single_number(vec![2, 2, 1]));
}

struct Solution {}
impl Solution {
    // bit magic is the solution that does achieve linear time and constant space
    pub fn single_number(nums: Vec<i32>) -> i32 {
        nums.iter().fold(0, |acc, num| acc ^ num)
    }

    // use std::collections::HashMap;
    // pub fn single_number(nums: Vec<i32>) -> i32 {
    //     let mut map: HashMap<i32, bool> = HashMap::new();
    //     for num in nums {
    //         map.entry(num)
    //             .and_modify(|has_occurred| *has_occurred = true)
    //             .or_insert(false);
    //     }
    //     map.into_iter()
    //         .find(|(_, has_occurred)| !*has_occurred)
    //         .unwrap()
    //         .0
    // }
}
