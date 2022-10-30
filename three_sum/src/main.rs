fn main() {
    // println!("{:?}", Solution::three_sum(vec![-1, 0, 1, 2, -1, -4]));
    println!(
        "{:?}",
        Solution::three_sum(vec![-4, -2, 1, -5, -4, -4, 4, -2, 0, 4, 0, -2, 3, 1, -5, 0])
    );
    // println!("{:?}", Solution::three_sum(vec![1, 1, -2]));
}

struct Solution {}
impl Solution {
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut nums = nums.clone();
        nums.sort();
        let index_of_first_non_negative_or_last_index = nums
            .iter()
            .enumerate()
            .find(|(_, val)| **val >= 0)
            .unwrap_or((nums.len() - 1, &0))
            .0;
        let mut tuple_set = std::collections::HashSet::new();
        for i in 0..(nums.len() - 2) {
            if nums[i] > 0 {
                break;
            }
            for j in (i + 1)..(nums.len() - 1) {
                for k in (j + 1).max(index_of_first_non_negative_or_last_index)..(nums.len()) {
                    if nums[k] < 0 {
                        continue;
                    }
                    if nums[i] + nums[j] + nums[k] == 0 {
                        let mut v = vec![nums[i], nums[j], nums[k]];
                        v.sort();
                        tuple_set.insert((v[0], v[1], v[2]));
                    }
                }
            }
        }
        tuple_set.iter().map(|z| vec![z.0, z.1, z.2]).collect()
    }
}

// the above works, but the time limit gets exceeded

// impl Solution {
//     pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
//         let mut nums: Vec<i32> = nums.iter().cloned().collect();
//         nums.sort();
//         // println!("length: {}, {nums:?}", nums.len());

//         // gotta sort
//         // gotta iterate i over all negatives and non-positives
//         // gotta iterate j over everything between i and whatever nums.len()-1
//         // gotta iterate k over all non-negatives

//         let last_i_exclusive = nums
//             .iter()
//             .enumerate()
//             .filter(|(_index, val)| **val <= 0)
//             .rev()
//             .next()
//             .unwrap_or((1, &0))
//             .0
//             .min(nums.len() - 3)
//             + 1;

//         let first_k = nums
//             .iter()
//             .enumerate()
//             .filter(|(_index, val)| **val >= 0)
//             .next()
//             .unwrap_or((nums.len() - 1, &0))
//             .0;

//         // let index_of_first_positive = nums
//         //     .iter()
//         //     .enumerate()
//         //     .skip(1)
//         //     // .inspect(|(index, val)| println!("{index}, {val}"))
//         //     .find(|(index, val)| nums[*index - 1] <= 0 && **val > 0)
//         //     .unwrap_or((nums.len(), &0))
//         //     .0;
//         // let index_of_last_nonpositive = if index_of_first_positive == nums.len() {
//         //     index_of_first_positive - 1
//         // } else {
//         //     nums.len() - 1
//         // };

//         // let index_of_first_k = index_of_first_positive.min(index_of_last_nonpositive);
//         // println!("{index_of_first_positive}, {index_of_last_nonpositive}, {index_of_first_k}");

//         // println!(
//         //     "len: {}, index_of_first_positive: {}",
//         //     nums.len(),
//         //     index_of_first_positive
//         // );

//         let mut tuple_set = std::collections::HashSet::new();
//         // for i in 0..(index_of_first_k - 1) {
//         for i in 0..last_i_exclusive {
//             for j in (i + 1)..(nums.len() - 1) {
//                 // for k in (j + 1).max(index_of_first_positive)..(nums.len()) {
//                 for k in (j + 1).max(first_k)..(nums.len()) {
//                     // println!("{i}, {j}, {k}");
//                     if nums[i] + nums[j] + nums[k] == 0 {
//                         let mut v = vec![nums[i], nums[j], nums[k]];
//                         v.sort();
//                         tuple_set.insert((v[0], v[1], v[2]));
//                     }
//                 }
//             }
//         }
//         tuple_set.iter().map(|z| vec![z.0, z.1, z.2]).collect()
//     }
// }
