fn main() {
    assert_eq!(Solution::num_decodings("0".into()), 0);
    assert_eq!(Solution::num_decodings("00".into()), 0);

    assert_eq!(Solution::num_decodings("7".into()), 1);
    assert_eq!(Solution::num_decodings("3".into()), 1);
    assert_eq!(Solution::num_decodings("1".into()), 1);
    assert_eq!(Solution::num_decodings("10".into()), 1);
    assert_eq!(Solution::num_decodings("33".into()), 1);

    assert_eq!(Solution::num_decodings("12".into()), 2);
    assert_eq!(Solution::num_decodings("226".into()), 3);
    assert_eq!(Solution::num_decodings("06".into()), 0);
    assert_eq!(Solution::num_decodings("11106".into()), 2);
    assert_eq!(
        Solution::num_decodings("111111111111111111111111111111111111111111111".into()),
        1836311903
    );
}

//

struct Solution {}
use std::collections::HashMap;
impl Solution {
    pub fn num_decodings(s: String) -> i32 {
        // println!("{s}");

        //// brute force is too slow
        // _num_decodings_recursive_brute_force(&s) as i32

        // let's use dynamic programming
        // this is unusual for me because it's bottom-up
        let mut subsolutions: HashMap<usize, usize> = HashMap::new();
        // subsolutions.insert(s.len() - 1, 1);
        for i in (0..s.len()).rev() {
            // _solve_with_memoization(&mut subsolutions, &s, i);
            go(&mut subsolutions, &s, i);
            // println!("subsolution for {} is {:?}", &s[i..], subsolutions.get(&i));
        }
        *subsolutions.get(&0).unwrap() as i32
    }
}

/*
it's dynamic programming

I go right to left

this is bottom-up instead of top-down

the only distinction is that I have to iterate every time I lift the bottom. It doesn't just recursively explode downward

I pass the hashmap, the whole slice, and the substring starting index

if I have the solution, cool

if the starting index is out of bounds, the solution is 0

if the substring is one character, the solution is 0 if the substring is "0", otherwise it's 1

otherwise I know that the substring is at least two characters

I can consider s[i..i+1]
I can consider s[i..i+2]
I can consider the solution for i+1
I _may_ be able to consider the solution for i+2

if s[i..i+1] is valid, then the solution is at least the solution for i+1
if s[i..i+2] is valid, then either i+2 is the end of the string or it isn't
  if i+2 is the last index, then we must add 1 to the solution because this is the first time we're seeing that solution at all
  else, we must add the solution for i+2 to the solution
 */

fn go(subsolutions: &mut HashMap<usize, usize>, s: &str, i: usize) -> usize {
    if let Some(num) = subsolutions.get(&i) {
        return *num;
    }

    if i == s.len() {
        return 0;
    }

    let substring = s.get(i..).unwrap();
    // println!("substring: {substring}");
    if substring.len() == 1 {
        let num = match substring {
            "0" => 0,
            _ => 1,
        };
        subsolutions.insert(i, num);
        return num;
    }

    let mut num = 0;

    // if the digit can be encoded on its own, we haven't stumbled onto any branches for encoding the substring
    if &s[i..i + 1] != "0" {
        num += go(subsolutions, s, i + 1);
    }

    // if the digit is 1 or 2 specifically, we need to consider whether it and the following digit can be encoded. If so, this is a new branch to consider, so we'll add the number of corresponding subsolutions to num
    match &s[i..i + 1] {
        // any digit can follow 1 to create an encodable two-digit number
        "1" => {
            // if these two digits are the very last substring of the whole slice, this is a new branch that stops right here and doesn't have a subsolution
            // else we can just fall back to the corresponding subsolution
            if i + 2 == s.len() {
                num += 1
            } else {
                num += go(subsolutions, s, i + 2);
            }
        }
        // only some digits can follow 2 to create an encodable two-digit number
        "2" => match &s[i + 1..i + 2] {
            "0" | "1" | "2" | "3" | "4" | "5" | "6" => {
                // this is copied from the match arm for "1" as the first digit above
                // I could have factored this into a closure or function, but whatever
                if i + 2 == s.len() {
                    num += 1
                } else {
                    num += go(subsolutions, s, i + 2);
                }
            }
            _ => {}
        },
        _ => {}
    }

    subsolutions.insert(i, num);

    num
}

// too slow
fn _num_decodings_recursive_brute_force(s: &str) -> usize {
    if s.is_empty() {
        return 1;
    }
    // println!("given {s}");
    match &s[..1] {
        "1" => {
            // on its own, "1" can be decoded
            // if it's followed by any digit, that too can be decoded
            _num_decodings_recursive_brute_force(&s[1..])
                + s.get(2..)
                    .map_or(0, |slice| _num_decodings_recursive_brute_force(slice))
        }
        "2" => {
            // on its own, "2" can be decoded
            // if it's followed by digits 0-6, that too can be decoded
            _num_decodings_recursive_brute_force(&s[1..])
                + s.get(1..2).map_or(0, |slice| {
                    // println!("considering slice {slice}");
                    match slice {
                        "0" | "1" | "2" | "3" | "4" | "5" | "6" => {
                            _num_decodings_recursive_brute_force(&s[2..])
                        }
                        _ => 0,
                    }
                })
        }
        "0" => 0,
        _ => _num_decodings_recursive_brute_force(&s[1..]),
    }
}
