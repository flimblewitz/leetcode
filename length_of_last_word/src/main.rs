fn main() {
    println!("{:?}", Solution::length_of_last_word("Hello World".into()));
    println!(
        "{:?}",
        Solution::length_of_last_word("   fly me   to   the moon  ".into())
    );
}

struct Solution {}
impl Solution {
    pub fn length_of_last_word(s: String) -> i32 {
        s.split_whitespace().last().map(|s| s.len()).unwrap() as i32
    }
}
