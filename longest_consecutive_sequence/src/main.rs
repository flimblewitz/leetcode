fn main() {
    println!(
        "{}",
        Solution::longest_consecutive(vec![100, 4, 200, 1, 3, 2])
    );
    println!(
        "{}",
        Solution::longest_consecutive(vec![0, 3, 7, 2, 5, 8, 4, 6, 0, 1])
    );
}

struct Solution;

use std::collections::HashSet;
impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        let nums: HashSet<i32> = nums.into_iter().collect();
        nums.iter()
            // filter to just the numbers that are the beginning of their own sequence
            .filter(|&&num| !nums.contains(&(num - 1)))
            // map each starting sequence number to the length of its sequence
            .map(|&num| {
                (num..)
                    .take_while(|consecutive_num| nums.contains(consecutive_num))
                    .count()
            })
            .max()
            .unwrap_or(0) as i32
    }
}

// fn naive(mut nums: Vec<i32>) -> i32 {
//     nums.sort_unstable();
//     let mut n = nums.into_iter();
//     let mut max_seq_len = 0;
//     let mut current_seq_len = 0;
//     let mut prev: Option<i32> = None;
//     while let Some(num) = n.next() {
//         match prev {
//             Some(p) => {
//                 if p + 1 == num {
//                     current_seq_len += 1;
//                 } else {
//                     current_seq_len = 1;
//                 }
//             }
//             None => {
//                 current_seq_len += 1;
//             }
//         }
//         max_seq_len = max_seq_len.max(current_seq_len);
//         prev = Some(num);
//     }
//     max_seq_len
// }
