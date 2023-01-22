fn main() {
    println!("{}", Solution::is_happy(19));
    println!("{}", Solution::is_happy(2));
}

struct Solution;

use std::collections::HashSet;
impl Solution {
    pub fn is_happy(n: i32) -> bool {
        let mut observed_numbers: HashSet<i32> = HashSet::new();
        observed_numbers.insert(n);
        let mut previous = n;
        loop {
            // println!("{}", previous);
            let next = Self::get_digits(previous)
                .into_iter()
                .map(|digit| digit * digit)
                .sum();
            if previous == next {
                return if next == 1 { true } else { false };
            }
            if observed_numbers.contains(&next) {
                return false;
            }
            observed_numbers.insert(next);
            previous = next;
        }
    }
    fn get_digits(n: i32) -> Vec<i32> {
        n.to_string()
            .chars()
            .map(|c| c.to_digit(10).unwrap() as i32)
            .collect()
    }
}
