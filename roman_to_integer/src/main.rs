fn main() {
    println!("{}", Solution::roman_to_int("III".into()));
    println!("{}", Solution::roman_to_int("LVIII".into()));
    println!("{}", Solution::roman_to_int("MCMXCIV".into()));
    println!("{}", Solution::roman_to_int("MMMXLV".into()));
}

struct Solution;

/*
    Symbol       Value
    I             1
    V             5
    X             10
    L             50
    C             100
    D             500
    M             1000

    I can be placed before V (5) and X (10) to make 4 and 9.
    X can be placed before L (50) and C (100) to make 40 and 90.
    C can be placed before D (500) and M (1000) to make 400 and 900.

*/

// this is crude, but whatever. It let me remind myself that you can match on slices and you have to index them with ranges instead of usizes (you can't fetch a single item from a slice, you can only get a subslice)
impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        let mut acc = 0;

        let mut i = 0;
        let len = s.len();

        while i < len {
            if i + 1 < len {
                let increase = match &s[i..i + 2] {
                    "CM" => 900,
                    "CD" => 400,
                    "XC" => 90,
                    "XL" => 40,
                    "IX" => 9,
                    "IV" => 4,
                    _ => 0,
                };
                if increase > 0 {
                    // println!("acc += {increase}, i += 2");
                    acc += increase;
                    i += 2;
                    continue;
                }
            }
            if i + 2 < len {
                let increase = match &s[i..i + 3] {
                    "MMM" => 3000,
                    "CCC" => 300,
                    "XXX" => 30,
                    "III" => 3,
                    _ => 0,
                };
                if increase > 0 {
                    // println!("acc += {increase}, i += 3");
                    acc += increase;
                    i += 3;
                    continue;
                }
            }
            if i + 1 < len {
                let increase = match &s[i..i + 2] {
                    "MM" => 2000,
                    "CC" => 200,
                    "XX" => 20,
                    "II" => 2,
                    _ => 0,
                };
                if increase > 0 {
                    // println!("acc += {increase}, i += 2");
                    acc += increase;
                    i += 2;
                    continue;
                }
            }
            if i < len {
                let increase = match &s[i..i + 1] {
                    "M" => 1000,
                    "D" => 500,
                    "C" => 100,
                    "L" => 50,
                    "X" => 10,
                    "V" => 5,
                    "I" => 1,
                    unexpected_substring => panic!("didn't expect {}", unexpected_substring),
                };
                acc += increase;
                i += 1;
                // println!("acc += {increase}, i += 1");
                continue;
            }
        }

        acc
    }
}
