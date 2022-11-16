fn main() {
    // println!("{:?}", Solution::my_sqrt(0));
    // println!("{:?}", Solution::my_sqrt(1));
    // println!("{:?}", Solution::my_sqrt(2));
    // println!("{:?}", Solution::my_sqrt(3));
    // println!("{:?}", Solution::my_sqrt(4));
    // println!("{:?}", Solution::my_sqrt(5));
    println!("{:?}", Solution::my_sqrt(8));
    println!("{:?}", Solution::my_sqrt(2147395600)); //46340 ... 46341
    println!("{:?}", Solution::my_sqrt(2147483647)); //46340 ... 2147483647
}
struct Solution {}
impl Solution {
    pub fn my_sqrt(x: i32) -> i32 {
        for i in 1..(x.checked_add(1).unwrap_or(std::i32::MAX)) {
            match i.checked_mul(i) {
                None => return i - 1, // x is the biggest square i32
                // Some(square) if square == x => return i,
                Some(square) if square > x => return i - 1, // if i is the root of the next biggest square, use i-1
                _ => continue,
            }
        }
        x
    }
}
