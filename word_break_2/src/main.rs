fn main() {
    println!(
        "{:?}",
        Solution::word_break(
            "catsanddog".into(),
            vec![
                "cat".into(),
                "cats".into(),
                "and".into(),
                "sand".into(),
                "dog".into()
            ]
        )
    );
    println!(
        "{:?}",
        Solution::word_break(
            "pineapplepenapple".into(),
            vec![
                "apple".into(),
                "pen".into(),
                "applepen".into(),
                "pine".into(),
                "pineapple".into()
            ]
        )
    );
}

struct Solution {}
impl Solution {
    pub fn word_break(s: String, word_dict: Vec<String>) -> Vec<String> {
        recursively_solve(&s, &word_dict)
            .unwrap_or(vec![])
            .into_iter()
            .filter_map(|v| Some(v))
            .collect()
    }
}

fn recursively_solve(s: &str, word_dict: &Vec<String>) -> Option<Vec<String>> {
    let mut v = vec![];
    for i in 0..s.len() {
        if word_dict.contains(&s[..(i + 1)].into()) {
            if i + 1 == s.len() {
                v.push(s.into());
            } else {
                let mut sub_solutions = recursively_solve(&s[(i + 1)..], word_dict)?
                    .iter_mut()
                    .map(|ss| format!("{} {}", &s[..(i + 1)], ss))
                    .collect();
                v.append(&mut sub_solutions);
            }
        }
    }
    Some(v)
}
