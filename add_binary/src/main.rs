fn main() {
    assert_eq!(Solution::add_binary("11".into(), "1".into()), "100");
    assert_eq!(Solution::add_binary("11111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111".into(), "1".into()), "100000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000");
}

struct Solution {}
impl Solution {
    pub fn add_binary(a: String, b: String) -> String {
        // the easy way to do this is just to use from_str_radix, but the problem cheekily gives us binary numbers that are too big to fit in even the biggest numerical data type
        // let l = u128::from_str_radix(&a, 2).unwrap();
        // let r = u128::from_str_radix(&b, 2).unwrap();
        // format!("{:b}", l + r)

        let mut carry = false;
        let mut a_chars = a.chars().rev();
        let mut b_chars = b.chars().rev();
        let mut sum = String::new();
        loop {
            let a_char = a_chars.next();
            let b_char = b_chars.next();
            if !carry && a_char.is_none() && b_char.is_none() {
                break;
            }

            // println!("{carry}, {a_char:?}, {b_char:?}");

            let c = match (
                carry,
                a_char.map_or(false, |c| c == '1'),
                b_char.map_or(false, |c| c == '1'),
            ) {
                (true, true, true) => '1',
                (true, true, false) | (true, false, true) | (false, true, true) => {
                    carry = true;
                    '0'
                }
                (true, false, false) | (false, true, false) | (false, false, true) => {
                    carry = false;
                    '1'
                }
                (false, false, false) => '0',
            };
            // println!("{carry}, {c}");
            sum.push(c)
        }
        sum.chars().rev().collect()
    }
}
