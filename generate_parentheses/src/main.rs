fn main() {
    println!("{:?}", Solution::generate_parenthesis(3));
}

struct Solution {}
impl Solution {
    // leetcode misspelled this fn name :shrug:
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        recursively_generate_parentheses("".into(), n, n)
    }
}

// enum Par {
//     Open,
//     Close,
// }
// struct ParTree {
//     num_open_pars: usize,
//     combinations: Vec<String>,
// }
fn recursively_generate_parentheses(
    s: String,
    unused_opens: i32,
    unused_closes: i32,
) -> Vec<String> {
    let mut v = vec![];
    if unused_opens > 0 {
        let mut new_s = s.clone();
        new_s.push('(');
        let mut possibilities =
            recursively_generate_parentheses(new_s, unused_opens - 1, unused_closes);
        v.append(&mut possibilities);
    }
    if unused_opens < unused_closes {
        let mut new_s = s.clone();
        new_s.push(')');
        let mut possibilities =
            recursively_generate_parentheses(new_s, unused_opens, unused_closes - 1);
        v.append(&mut possibilities);
    }
    if unused_opens == 0 && unused_closes == 0 {
        v.push(s);
    }
    v
}
