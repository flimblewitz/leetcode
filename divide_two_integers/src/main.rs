fn main() {
    println!("{:?}", Solution::divide(10, 3));
    println!("{:?}", Solution::divide(7, -3));
    println!("{:?}", Solution::divide(-2147483648, -1)); // 2147483647
    println!("{:?}", Solution::divide(-2147483648, 1)); // -2147483648
    println!("{:?}", Solution::divide(2147483647, 2)); // 1073741823
    println!("{:?}", Solution::divide(-2147483648, 2)); //-1073741824
    println!("{:?}", Solution::divide(2147483647, -2147483648)); //0
}

struct Solution {}
impl Solution {
    pub fn divide(dividend: i32, divisor: i32) -> i32 {
        let mut quotient = 0;
        let is_negative = match (dividend < 0, divisor < 0) {
            (true, false) => true,
            (false, true) => true,
            _ => false,
        };
        // let mut remainder = dividend.checked_abs().unwrap_or(std::i32::MAX);
        let (mut remainder, dividend_was_min) = match dividend.checked_abs() {
            Some(abs_val) => (abs_val, false),
            None => (std::i32::MAX, true),
        };
        // let abs_divisor = divisor.checked_abs().unwrap_or(std::i32::MAX);
        let (abs_divisor, divisor_was_min) = match divisor.checked_abs() {
            Some(abs_val) => (abs_val, false),
            None => (std::i32::MAX, true),
        };
        if !dividend_was_min && divisor_was_min {
            return 0;
        }
        // dumb optimization
        if abs_divisor == 1 {
            quotient = remainder;
            // if this isn't good enough, it's time to do bit math
        } else {
            while remainder >= abs_divisor {
                quotient += 1;
                remainder -= abs_divisor;
            }
            // overflow fun times
            if dividend_was_min {
                if remainder + 1 >= abs_divisor {
                    quotient += 1;
                }
            }
        }
        match (quotient, is_negative) {
            (std::i32::MAX, true) => std::i32::MIN,
            (_, true) => -quotient,
            _ => quotient,
        }
    }
}
