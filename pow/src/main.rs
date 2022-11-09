fn main() {
    println!("{:?}", Solution::my_pow(2.0, 14));
    println!("{:?}", Solution::my_pow(2.0, 10));
    println!("{:?}", Solution::my_pow(2.0, 0));
    println!("{:?}", Solution::my_pow(2.0, -2));
    println!("{:?}", Solution::my_pow(8.84372, -5));
    // time limit ahoy
    // println!("{}, {}", std::i32::MAX, 2147483647 == std::i32::MAX);
    println!("{:?}", Solution::my_pow(0.00001, 2147483647));
    println!("{:?}", Solution::my_pow(2.00000, -2147483648));
}

// acc = x = 2, n=10
// 2^10 = (2^2)^5 = (2^2)*(2^2)^4 = (2^2)*((2^2)^2)^2

// this is mine and beats 100% runtime and 97.89% memory (okay, memory varies wildly)
struct Solution {}
impl Solution {
    pub fn my_pow(x: f64, n: i32) -> f64 {
        let mut x = x;
        let mut abs_n = n;
        if n == 0 {
            return 1.0;
        } else if n < 0 {
            // let's pretend they gave us a positive n by changing x to x^-1
            x = 1.0 / x;
            // I just want to make n positive so that I can loop and reduce it to 1. If it was std::i32::MIN, I'll have to do another multiplication at the very end.
            abs_n = abs_n.checked_abs().unwrap_or(std::i32::MAX);
        }
        // the acc will be our accumulating product. It starts as x^1, which is just x
        let mut acc = x;
        // prev will be the most recent nested expression (we will be trying to halve abs_n, and it will be the nested expression that gets squared when that happens)
        let mut prev = x;
        while abs_n > 1 {
            // as long as abs_n is more than 1, we can take the "current" exponented expression (prev) and multiply it into the accumulator
            acc = acc * prev;
            match abs_n % 2 {
                // if abs_n is even, we will square prev because the acc just went from y*(prev^abs_n) to y*(prev^2)^(abs_n/2)
                0 => {
                    prev = prev * prev;
                    abs_n = abs_n / 2;
                }
                // if abs_n is odd, we will decrement prev because the acc just went from y*(prev^abs_n) to y*prev*(prev^(abs_n-1))
                _ => {
                    abs_n = abs_n - 1;
                }
            }
        }
        // since I used abs_n, if n was std::i32::MIN, that means abs_n was really 1 less than it should have been, which means I need to multiply x into the accumulator one more time
        if n == std::i32::MIN {
            return acc * x;
        }
        acc
    }
}
//// fancy bit magic. beats 100% runtime, 68.42% memory
// impl Solution {
//     pub fn my_pow(x: f64, n: i32) -> f64 {
//         let s = format!("{:b}", n.abs());
//         // println!("{s}");
//         let bits = s.chars().skip(1);
//         if n == 0 {
//             return 1.0;
//         } else if n > 0 {
//             return bits.fold(
//                 x,
//                 |acc, bit| if bit == '1' { acc * acc * x } else { acc * acc },
//             );
//         } else {
//             return bits.fold(1.0 / x, |acc, bit| {
//                 if bit == '1' {
//                     acc * acc * (1.0 / x)
//                 } else {
//                     acc * acc
//                 }
//             });
//         }
//     }
// }

//// brute force in one line but unnecessary checks
// impl Solution {
//     pub fn my_pow(x: f64, n: i32) -> f64 {
//         (0..n.abs()).fold(1.0, |acc, _| if n > 0 { acc * x } else { acc / x })
//     }
// }
//// brute force in more lines but fewer checks
// impl Solution {
//     pub fn my_pow(x: f64, n: i32) -> f64 {
//         match n >= 0 {
//             true => (0..n).fold(1.0, |acc, _| acc * x),
//             false => (n..0).fold(1.0, |acc, _| acc / x),
//         }
//     }
// }
