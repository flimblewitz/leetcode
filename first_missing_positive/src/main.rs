fn main() {
    println!("{:?}", Solution::first_missing_positive(vec![1, 2, 0]));
    // println!("{:?}", Solution::first_missing_positive(vec![3, 4, -1, 1]));
    // println!(
    //     "{:?}",
    //     Solution::first_missing_positive(vec![7, 8, 9, 11, 12])
    // );
    // println!(
    //     "{:?}",
    //     Solution::first_missing_positive(vec![100000, 1000002, -1, 1])
    // );
    // println!(
    //     "{:?}",
    //     Solution::first_missing_positive(vec![2147483647, 65538])
    // );
}

struct Solution {}
impl Solution {
    pub fn first_missing_positive(nums: Vec<i32>) -> i32 {
        todo!("sucks to suck")
        // assume 32 bit os
        // that means I can make arrays of u32s
        // each positive i32 is between 1 and 2^31-1 AKA 2147483647
        // in a single u32, I can represent numbers 1 to 32
        // so I need n = (2^31-1)/32 numbers, which is... a lot. So many that I can't have an array that long
        // I can, however, have m = std::i32::MAX / 2_usize.pow(6) as my array length, where m = n/2
        // so if I want to store n u32s, then I can store m tuples that each have 2 u32s

        // println!("number of u32s: {}", std::i32::MAX / 32);
        // let num_buckets = std::i32::MAX as usize / 2_usize.pow(6);
        // let mut buckets = [(0_u32, 0_u32); std::i32::MAX as usize / 2_usize.pow(6)];
        // for &num in nums.iter().filter(|n| **n > 0 && **n < std::i32::MAX) {
        //     let bucket_index = num as usize / 2_usize.pow(6);
        //     let remainder = num as usize % 2_usize.pow(6);
        //     let tuple_index = remainder / 2_usize.pow(5);
        //     let tuple_remainder = remainder as u32 % 2_u32.pow(5);
        //     buckets[bucket_index] = match tuple_index {
        //         0 => (0, tuple_remainder),
        //         1 => (tuple_remainder, 0),
        //         _ => panic!("um"),
        //     };
        // }

        // // let mut i = 1;
        // // for bucket in buckets.iter() {
        // for i in 0..std::i32::MAX {
        //     let bucket_index = i as usize / 2_usize.pow(6);
        //     let remainder = i as usize % 2_usize.pow(6);
        //     let tuple_index = remainder / 2_usize.pow(5);
        //     let tuple_remainder = remainder as u32 % 2_u32.pow(5);
        //     let found = match tuple_index {
        //         0 => tuple_remainder == buckets[bucket_index].0,
        //         1 => tuple_remainder == buckets[bucket_index].1,
        //         _ => panic!("oof"),
        //     };
        //     if found {
        //         return i;
        //     }
        //     // i += 1;
        // }
        // std::i32::MAX

        // todo!("bluh")

        // // let mut used_numbers = vec![false; std::i32::MAX as usize];
        // // let mut used_numbers = vec![false; std::i64::MAX as usize];
        // let num_buckets = std::i32::MAX as usize / std::i16::MAX as usize;
        // // println!("num_buckets: {}", num_buckets);
        // // let mut used_numbers = vec![vec![false; std::i16::MAX as usize]; num_buckets];
        // let mut used_numbers =
        //     [[false; std::i16::MAX as usize]; (std::i32::MAX as usize / std::i16::MAX as usize)];
        // for &num in nums.iter() {
        //     if num < 1 {
        //         continue;
        //     }
        //     let j = num as usize / std::i16::MAX as usize;
        //     let i = num as usize % std::i16::MAX as usize;
        //     // println!("{num}, {j}, {i}",);
        //     used_numbers[j.min(num_buckets - 1)][i] = true;
        // }
        // for j in 0..num_buckets {
        //     let mut i = 1;
        //     while i < std::i32::MAX {
        //         if !used_numbers[j][i as usize] {
        //             return i;
        //         }
        //         i += 1
        //     }
        // }
        // std::i32::MAX

        //// ugh
        // let mut used_numbers = vec![false; std::i32::MAX as usize];
        // for &num in nums.iter().filter(|n| **n > 0) {
        //     used_numbers[num as usize] = true;
        // }

        // let mut i = 1;
        // while i < std::i32::MAX {
        //     if !used_numbers[i as usize] {
        //         return i;
        //     }
        //     i += 1
        // }
        // i
    }
}

// used_numbers
//     .iter()
//     .enumerate()
//     .skip(1)
//     .filter(|(_, used)| !**used)
//     .next()
//     .unwrap()
//     .0 as i32

// let mut acc: u32 = 0b0;

// 10^5 == 100000
// I have 32 bits to play with in a u32
// 100000 / 32 == 3125
// ergo I need 3125 u32s

// let mut used_numbers = [false; std::i32::MAX as usize];
// let mut used_numbers = [false; 5];
// let mut used_numbers = vec![false; 2147483647];

// let num_buckets = std::i32::MAX / 32;
// let mut accs = [0_u32; (std::i32::MAX / 32) as usize];

// so 10^5 is a big number
// really I can only do this bit shifty stuff for numbers that are up to 31
// so I can't use one acc, I have to use a constant pool of them for different ranges

// 2^100000

// for &num in nums.iter().filter(|n| **n > 0) {
//     // acc = acc | 1 << (num - 1);
//     let bucket = (num / num_buckets) as usize;
//     // let index_in_bucket = num % num_buckets - 1;
//     let index_in_bucket = if num < num_buckets {
//         num - 1
//     } else {
//         num % num_buckets
//     };
//     accs[bucket] = accs[bucket] | 1 << index_in_bucket;
// }

// let mut i = 1;
// 'outer: for acc in accs.iter() {
//     let s = format!("{:#032b}", acc);
//     for c in s.chars().rev() {
//         if c == '0' {
//             break 'outer;
//         }
//         i += 1;
//     }
// }
// i
