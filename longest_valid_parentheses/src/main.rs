fn main() {
    println!("{:?}", Solution::longest_valid_parentheses("(()".into()));
    println!("{:?}", Solution::longest_valid_parentheses(")()())".into()));
    println!(
        "{:?}",
        Solution::longest_valid_parentheses("(((()()))".into())
    );
    // println!(
    //     "{:?}",
    //     Solution::longest_valid_parentheses("(((((()((((()((".into())
    // );
    println!(
        "{:?}",
        Solution::longest_valid_parentheses("((())())".into())
    );
}

struct Solution {}
impl Solution {
    pub fn longest_valid_parentheses(s: String) -> i32 {
        let mut indexes_of_paired_pars = vec![None; s.len()];
        let mut unpaired_open_par_indexes = vec![];
        for (i, c) in s.chars().enumerate() {
            match c {
                '(' => unpaired_open_par_indexes.push(i),
                ')' => {
                    if let Some(index_of_last_unpaired_par) =
                        unpaired_open_par_indexes.iter().rev().next()
                    {
                        indexes_of_paired_pars[*index_of_last_unpaired_par] = Some(i);
                        indexes_of_paired_pars[i] = Some(*index_of_last_unpaired_par);
                        unpaired_open_par_indexes.pop();
                    }
                }
                _ => panic!("there's a non-parenthesis char {c} at index {i}"),
            }
        }

        // println!("{:?}", indexes_of_paired_pars);

        let mut longest_span_length = 0;
        let mut left = 0;
        while left < s.len() {
            if indexes_of_paired_pars[left].is_none() {
                left += 1;
                continue;
            }
            let mut right = left;
            loop {
                // if we're out of bounds, time to do math
                // if we've hit a par who has no mate, time to do math
                // if we just hit a close par whose mate lies beyond left, time do to math
                match indexes_of_paired_pars.get(right) {
                    None => {
                        break;
                    }
                    Some(None) => {
                        break;
                    }
                    Some(Some(mate_index)) if *mate_index < left => {
                        break;
                    }
                    _ => {
                        right += 1;
                    }
                }
            }
            for i in left..right {
                if let Some(mate_index) = indexes_of_paired_pars[i] {
                    if mate_index >= right {
                        left = i;
                    }
                }
            }
            longest_span_length = longest_span_length.max(right - left);
            left = right;
        }
        longest_span_length as i32
    }
}
