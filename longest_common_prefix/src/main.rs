fn main() {
    println!(
        "{:?}",
        Solution::longest_common_prefix(vec!["flower".into(), "flow".into(), "flight".into()])
    );
    println!(
        "{:?}",
        Solution::longest_common_prefix(vec!["flower".into(), "flow".into(), "flog".into()])
    );
    println!(
        "{:?}",
        Solution::longest_common_prefix(vec!["dog".into(), "racecar".into(), "car".into()])
    );
    println!("{:?}", Solution::longest_common_prefix(vec!["dog".into()]));
}

struct Solution {}
impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        strs.iter()
            .fold(None, |acc: Option<String>, s| match acc {
                Some(acc) => Some(get_lcp(&acc, &s)),
                None => Some(s.into()),
            })
            .unwrap()
    }
}

fn get_lcp(a: &str, b: &str) -> String {
    a.chars()
        .zip(b.chars())
        .take_while(|(left, right)| left == right)
        .map(|(left, _)| left)
        .collect()
}
