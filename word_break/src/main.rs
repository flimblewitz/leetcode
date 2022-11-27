fn main() {
    println!(
        "{:?}",
        Solution::word_break("ab".into(), vec!["a".into(), "b".into()])
    );
    println!(
        "{:?}",
        Solution::word_break("abb".into(), vec!["a".into(), "bb".into()])
    );
    println!(
        "{:?}",
        Solution::word_break("abbb".into(), vec!["a".into(), "bb".into()])
    );
    println!(
        "{:?}",
        Solution::word_break("leetcode".into(), vec!["leet".into(), "code".into()])
    );
}

struct Solution {}
use std::collections::HashMap;
impl Solution {
    pub fn word_break(s: String, word_dict: Vec<String>) -> bool {
        let mut sub_solutions: HashMap<String, bool> = HashMap::new();
        recursively_solve(&mut sub_solutions, &s, &word_dict)
    }
}

fn recursively_solve(
    sub_solutions: &mut HashMap<String, bool>,
    s: &str,
    word_dict: &Vec<String>,
) -> bool {
    if let Some(solution) = sub_solutions.get(s) {
        return *solution;
    }
    if word_dict.contains(&s.into()) {
        sub_solutions.insert(s.into(), true);
        return true;
    }
    // we want to check if &s[..i] is in the word dict and &s[i..] can itself be broken up into recognized words
    // we only want to do the latter if there's anything else to check
    for i in 1..s.len() {
        let remainder_is_good_or_empty = if i < s.len() {
            recursively_solve(sub_solutions, &s[i..], word_dict)
        } else {
            true
        };
        if remainder_is_good_or_empty && word_dict.contains(&s[..i].into()) {
            sub_solutions.insert(s.into(), true);
            return true;
        }
    }
    sub_solutions.insert(s.into(), false);
    false
}
