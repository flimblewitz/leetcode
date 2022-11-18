fn main() {
    println!("{:?}", Solution::gray_code(1)); // [0, 1]
    println!("{:?}", Solution::gray_code(2)); // [0,1,3,2]
    println!("{:?}", Solution::gray_code(3)); // [0,1,3,2,6,7,5,4], not [0,1,3,2,6,4,5,7]
    println!("{:?}", Solution::gray_code(4));
    println!("{:?}", Solution::gray_code(5));
    println!("{:?}", Solution::gray_code(6));
    println!("{:?}", Solution::gray_code(7));
    // println!("{:?}", Solution::gray_code(10));
}
struct Solution {}
// it turns out that there's a fancy trick. You can just take a reversed copy of the array, add a power of 2 to each item, and append that to the array. Repeat n times. Wow.
impl Solution {
    pub fn gray_code(n: i32) -> Vec<i32> {
        let mut x = 1;
        let mut ret = vec![0];

        for _ in 0..n {
            let mut rev = ret.iter().rev().map(|&num| num + x).collect();
            ret.append(&mut rev);
            x *= 2;
        }

        ret
    }
}
// my solution was not good and I had no idea how to solve this
// impl Solution {
//     pub fn gray_code(n: i32) -> Vec<i32> {
//         // every int other than 0 and 2^n - 1 has 2 adjacent partners and also a partner for every n bits
//         let mut used = vec![false; 2_usize.pow(n as u32)];
//         used[0] = true;
//         let mut v = vec![0; 2_usize.pow(n as u32)];
//         v[0] = 0;
//         // recursively_solve(&mut v, &mut used, 0);
//         let last_index = v.len() - 1;
//         let mut p = 0;
//         let mut adjacent_partner = v[0] + 2_i32.pow(p);
//         while adjacent_partner < v.len() as i32 {
//             if !used[adjacent_partner as usize] && (v[0] ^ adjacent_partner).count_ones() == 1 {
//                 used[adjacent_partner as usize] = true;
//                 v[last_index] = adjacent_partner;
//                 if recursively_solve(&mut v, &mut used, 0) {
//                     // return true;
//                     break;
//                 }
//                 v[last_index] = 0;
//                 used[adjacent_partner as usize] = false;
//             }
//             p += 1;
//             adjacent_partner = v[0] + 2_i32.pow(p);
//         }
//         v
//     }
// }

// fn recursively_solve(v: &mut [i32], used: &mut [bool], index: usize) -> bool {
//     if index >= v.len() - 2 {
//         return true;
//     }
//     // if index == v.len() - 1 {
//     //     // return true;
//     //     return (v[0] ^ v[v.len() - 1]).count_ones() == 1;
//     // }
//     // given current index, iterate over every number in bounds that's index plus or minus some power of n
//     // if that adjacent binary number hasn't been used, mark it as used, and increment the index. If the next call returns true, we're done. Otherwise, keep looping
//     let mut p = 0;
//     let mut adjacent_partner = v[index] - 2_i32.pow(p);
//     while adjacent_partner > 0 {
//         if !used[adjacent_partner as usize] && (v[index] ^ adjacent_partner).count_ones() == 1 {
//             used[adjacent_partner as usize] = true;
//             v[index + 1] = adjacent_partner;
//             if recursively_solve(v, used, index + 1) {
//                 return true;
//             }
//             v[index + 1] = 0;
//             used[adjacent_partner as usize] = false;
//         }
//         p += 1;
//         adjacent_partner = v[index] - 2_i32.pow(p);
//     }
//     let mut p = 0;
//     let mut adjacent_partner = v[index] + 2_i32.pow(p);
//     while adjacent_partner < v.len() as i32 {
//         if !used[adjacent_partner as usize] && (v[index] ^ adjacent_partner).count_ones() == 1 {
//             used[adjacent_partner as usize] = true;
//             v[index + 1] = adjacent_partner;
//             if recursively_solve(v, used, index + 1) {
//                 return true;
//             }
//             v[index + 1] = 0;
//             used[adjacent_partner as usize] = false;
//         }
//         p += 1;
//         adjacent_partner = v[index] + 2_i32.pow(p);
//     }
//     false
// }
