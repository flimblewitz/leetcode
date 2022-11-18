fn main() {
    println!("{:?}", Solution::restore_ip_addresses("25525511135".into()));
}
struct Solution {}
impl Solution {
    pub fn restore_ip_addresses(s: String) -> Vec<String> {
        recursively_solve(&mut s.clone(), 0, 3)
    }
}

fn recursively_solve(chars: &mut String, index: usize, remaining_periods: usize) -> Vec<String> {
    if index >= chars.len() {
        return vec![];
    }
    if remaining_periods == 0 {
        if digit_block_is_valid(chars, index, chars.len()) {
            return vec![chars.clone()];
        } else {
            return vec![];
        }
    }
    let mut v = vec![];
    for end in (index + 1)..=(index + 3).min(chars.len()) {
        if digit_block_is_valid(chars, index, end) {
            chars.insert(end, '.');
            v.append(&mut recursively_solve(
                chars,
                end + 1,
                remaining_periods - 1,
            ));
            chars.remove(end);
        }
    }
    v
}

fn digit_block_is_valid(chars: &String, start: usize, end: usize) -> bool {
    let digit_block: String = chars.clone()[start..end].into();
    match digit_block.parse::<u32>() {
        Ok(num) => num <= 255 && !(digit_block.len() > 1 && digit_block.starts_with("0")),
        _ => false,
    }
}
