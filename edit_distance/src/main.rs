use std::collections::HashMap;

fn main() {
    println!("{:?}", Solution::min_distance("horse".into(), "ros".into()));
    println!(
        "{:?}",
        Solution::min_distance("intention".into(), "execution".into())
    );
    println!(
        "{:?}",
        Solution::min_distance("food".into(), "money".into())
    ); // should be 4, not 3
    println!(
        "{:?}",
        Solution::min_distance("spartan".into(), "part".into())
    ); // should be 3, not 2
}
struct Solution {}
impl Solution {
    pub fn min_distance(word1: String, word2: String) -> i32 {
        let mut sub_solutions: HashMap<(String, String), i32> = HashMap::new();
        recursively_solve(&mut sub_solutions, &word1, &word2)
    }
}
fn recursively_solve(
    sub_solutions: &mut HashMap<(String, String), i32>,
    word1: &str,
    word2: &str,
) -> i32 {
    if let Some(solution) = sub_solutions.get(&(word1.into(), word2.into())) {
        return *solution;
    }
    let best_dist: i32 = match (word1.get(0..1), word2.get(0..1)) {
        (None, None) => 0,
        (Some(w1), Some(w2)) => {
            if *w1 == *w2 {
                return recursively_solve(sub_solutions, &word1[1..], &word2[1..]);
            } else {
                1 + recursively_solve(sub_solutions, &word1[1..], &word2[1..])
                    .min(recursively_solve(sub_solutions, &word1, &word2[1..]))
                    .min(recursively_solve(sub_solutions, &word1[1..], &word2))
            }
        }
        (Some(_), None) => word1.len() as i32,
        (None, Some(_)) => word2.len() as i32,
    };
    sub_solutions.insert((word1.into(), word2.into()), best_dist);
    // println!("{}, {}, {}", word1, word2, best_dist);
    best_dist
}
