fn main() {
    println!("{}", Solution::is_isomorphic("egg".into(), "add".into()));
    println!("{}", Solution::is_isomorphic("foo".into(), "bar".into()));
    println!(
        "{}",
        Solution::is_isomorphic("paper".into(), "title".into())
    );
}

struct Solution;

use std::collections::HashMap;
impl Solution {
    pub fn is_isomorphic(s: String, t: String) -> bool {
        /*
        so the strings need to be converted to another data type
        as I iterate over chars, I'll fill in a hashmap of observed chars and assign an incrementing usize value to it. Then I'll fill in a vec where the char has been converted to that usize
        I could reduce the memory consumption by replacing the hashmap with a vec and use the normalized ascii values as indexes, but I just don't care
        */
        Self::get_distinct_tokens(&s) == Self::get_distinct_tokens(&t)
    }
    fn get_distinct_tokens(s: &str) -> Vec<usize> {
        let mut distinct_chars_observed_counter = 0;
        let mut chars_to_order_of_observation: HashMap<char, usize> = HashMap::new();
        s.chars().fold(vec![], |mut acc, c| {
            let order_of_observation =
                *chars_to_order_of_observation.entry(c).or_insert_with(|| {
                    distinct_chars_observed_counter += 1;
                    distinct_chars_observed_counter
                });
            acc.push(order_of_observation);
            acc
        })
    }
}
