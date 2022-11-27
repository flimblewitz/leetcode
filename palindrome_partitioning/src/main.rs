fn main() {
    println!("{:?}", Solution::partition("aab".into()));
    println!("{:?}", Solution::partition("efe".into()));
}

struct Solution {}
use std::collections::HashMap;
impl Solution {
    pub fn partition(s: String) -> Vec<Vec<String>> {
        let mut sub_solutions: HashMap<String, Vec<Vec<String>>> = HashMap::new();
        recursively_get_palindrome_partitions(&mut sub_solutions, &s)
    }
}

fn recursively_get_palindrome_partitions(
    sub_solutions: &mut HashMap<String, Vec<Vec<String>>>,
    s: &str,
) -> Vec<Vec<String>> {
    if let Some(sub_solution) = sub_solutions.get(s) {
        return sub_solution.clone();
    }
    let mut v = vec![];
    for i in 0..s.len() {
        if is_palindrome(&s[..i + 1]) {
            if i + 1 < s.len() {
                let palindrome_partitions =
                    recursively_get_palindrome_partitions(sub_solutions, &s[i + 1..]);
                palindrome_partitions
                    .into_iter()
                    .for_each(|mut palindrome_partition| {
                        palindrome_partition.insert(0, s[..i + 1].into());
                        v.push(palindrome_partition)
                    });
            } else {
                v.push(vec![s[..i + 1].into()]);
            }
        }
    }
    v
}

fn is_palindrome(s: &str) -> bool {
    if s.is_empty() {
        panic!("don't pass in empty string")
    }
    match s.len() % 2 {
        0 => s[..s.len() / 2] == s[s.len() / 2..].chars().rev().collect::<String>(),
        _ => {
            s.len() == 1
                || s[..s.len() / 2] == s[s.len() / 2 + 1..].chars().rev().collect::<String>()
        }
    }
}
