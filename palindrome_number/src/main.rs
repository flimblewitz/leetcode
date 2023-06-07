fn main() {
    println!("{:?}", Solution::is_palindrome(121));
    println!("{:?}", Solution::is_palindrome(-121));
    println!("{:?}", Solution::is_palindrome(10));
    println!("{:?}", Solution::is_palindrome(11));
    println!("{:?}", Solution::is_palindrome(1000021));
    println!("{:?}", Solution::is_palindrome(10000201));
}

struct Solution;
impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        let x_chars: Vec<_> = x.to_string().chars().collect();
        is_char_slice_palindrome_recursive(&x_chars)
    }
}

fn is_char_slice_palindrome_recursive(x_chars: &[char]) -> bool {
    if x_chars.len() == 1 {
        return true;
    }

    if x_chars.first() != x_chars.last() {
        return false;
    }

    if x_chars.len() > 2 {
        let inner_chars = x_chars.get(1..(x_chars.len() - 1)).unwrap();
        return is_char_slice_palindrome_recursive(inner_chars);
    };

    true
}

// leetcode challenged me to solve this without converting to a string, so I started with a mathematical idea but it fell apart. It's so much simpler to just tokenize the digits
// this falls apart on numbers like 1000021 because the zeroes are suddenly discounted
// fn is_i32_palindrome_recursive(x: i32) -> bool {
//     if x < 0 {
//         return false;
//     }
//     if x < 10 {
//         return true;
//     }

//     // leetcode doesn't recognize ilog10()
//     // let greatest_power_of_10_in_x = 10_i32.pow(x.ilog10());

//     let greatest_power_of_10_in_x = 10_i32.pow(
//         (2_u32..)
//             .find(|exponent| x < 10_i32.pow(*exponent))
//             .unwrap()
//             - 1,
//     );
//     // println!("{x}, {greatest_power_of_10_in_x}");

//     let leftmost_digit = (x) / greatest_power_of_10_in_x;
//     let rightmost_digit = x % 10;

//     let x_without_leftmost_and_rightmost_digits = x % greatest_power_of_10_in_x / 10;
//     // println!("x_without_leftmost_and_rightmost_digits: {x_without_leftmost_and_rightmost_digits}");

//     leftmost_digit == rightmost_digit
//         && is_i32_palindrome_recursive(x_without_leftmost_and_rightmost_digits)
// }
