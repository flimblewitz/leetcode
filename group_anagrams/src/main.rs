use std::collections::HashMap;

fn main() {
    println!(
        "{:?}",
        Solution::group_anagrams(
            vec!["eat", "tea", "tan", "ate", "nat", "bat"]
                .into_iter()
                .map(|s| s.into())
                .collect()
        )
    );
    println!(
        "{:?}",
        Solution::group_anagrams(vec![""].into_iter().map(|s| s.into()).collect())
    );
    println!(
        "{:?}",
        Solution::group_anagrams(vec!["a"].into_iter().map(|s| s.into()).collect())
    );
    println!(
        "{:?}",
        Solution::group_anagrams(vec!["", "b"].into_iter().map(|s| s.into()).collect())
    );
}

struct Solution {}
impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut groups: HashMap<String, Vec<String>> = HashMap::new();
        for s in strs {
            let mut ss = s.clone().chars().collect::<Vec<char>>();
            ss.sort_unstable();
            groups.entry(ss.iter().collect()).or_insert(vec![]).push(s);
        }
        groups.into_values().collect()
    }
}
