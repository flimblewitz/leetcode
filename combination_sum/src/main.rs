// use std::collections::HashMap;

fn main() {
    // println!("{:?}", Solution::combination_sum(vec![1], 1));
    // println!("{:?}", Solution::combination_sum(vec![1, 2], 2));
    // println!("{:?}", Solution::combination_sum(vec![1, 2], 3));
    // println!("{:?}", Solution::combination_sum(vec![1, 2, 3], 4));
    println!("{:?}", Solution::combination_sum(vec![2, 3, 6, 7], 7));
}

struct Solution {}

impl Solution {
    pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut candidates = candidates.clone();
        candidates = candidates
            .iter()
            .filter(|item| **item <= target)
            .cloned()
            .collect();
        candidates.sort();

        recurse(&candidates, target, 0)
    }
}
fn recurse(candidates: &Vec<i32>, target: i32, index: usize) -> Vec<Vec<i32>> {
    let mut v = vec![];
    for i in index..(candidates.len()) {
        let c = candidates[i];
        let mut n = 1;
        while n * c < target {
            let mut partial_combinations = recurse(candidates, target - n * c, i + 1);
            if partial_combinations.len() > 0 {
                for pc in partial_combinations.iter_mut() {
                    pc.append(&mut vec![c; n as usize]);
                }
                v.append(&mut partial_combinations);
            }
            n += 1;
        }
        if n * c == target {
            v.push(vec![c; n as usize]);
        }
    }
    v
}

// my commented code below is for a dynamic programming attempt that I'm sure is possible and better, but brute force was better for this kind of quick homework problem

// impl Solution {
//     pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
//         // I want to filter out candidates that are greater than the target
//         let mut candidates = candidates.clone();
//         candidates = candidates
//             .iter()
//             .filter_map(|c| (*c <= target).then_some(*c))
//             .collect();
//         // sort candidates
//         candidates.sort();

//         // let mut combinations_with_target_sum = vec![];

//         // now I have to iterate over all combinations
//         // this is a dynamic programming question
//         // I want to build up a table of combinations that create smaller sums
//         // list of keys, will be sorted
//         let mut attainable_sums: Vec<i32> = vec![];
//         // let mut sum_combinations: HashMap<i32, Vec<Vec<i32>>> = HashMap::new();
//         let mut sum_combinations: HashMap<i32, Option<Vec<Vec<i32>>>> = HashMap::new();
//         // starting at the integer equal to combinations[0], I look for all possible sums
//         for &candidate in candidates.iter() {
//             attainable_sums.push(candidate);
//             let mut combinations_with_candidate_sum = vec![vec![candidate]];
//             // now iterate over every attainable_sum less than candidate and compose combinations
//             if let Some(mut found) = recursively_get_combinations_with_target_sum(
//                 candidate,
//                 &mut attainable_sums,
//                 &mut sum_combinations,
//             ) {
//                 combinations_with_candidate_sum.append(&mut found);
//             }
//             sum_combinations.insert(candidate, Some(combinations_with_candidate_sum));
//             println!("sum_combinations is now {:?}", sum_combinations);
//         }

//         // combinations_with_target_sum
//         // sum_combinations.get(target).unwrap_or(vec![vec![]])

//         // sum_combinations
//         //     .get(&target)
//         //     .and_then(|combinations| Some(combinations.clone()))
//         //     .unwrap_or_default()

//         // .and_then(|combinations| combinations.unwrap_or_default())
//         // .unwrap_or(vec![vec![]])

//         // if let Some(Some(combinations)) = sum_combinations.get(&target) {
//         //     return combinations.clone();
//         // }

//         if let Some(found) = recursively_get_combinations_with_target_sum(
//             target,
//             &mut attainable_sums,
//             &mut sum_combinations,
//         ) {
//             return found;
//         }

//         vec![vec![]]
//     }
// }

// fn recursively_get_combinations_with_target_sum(
//     target: i32,
//     attainable_sums: &mut Vec<i32>,
//     sum_combinations: &mut HashMap<i32, Option<Vec<Vec<i32>>>>,
// ) -> Option<Vec<Vec<i32>>> {
//     let mut combinations = vec![];
//     for attainable_sum in attainable_sums
//         .clone()
//         .iter()
//         .take_while(|attainable_sum| **attainable_sum <= target)
//     {
//         // we need to get all the ways to reach the target given all possible repeats of the attainable sum in the mix
//         for n in 1..=(target / (*attainable_sum)) {
//             let partial_target = target - n * *attainable_sum;
//             let mut combinations_with_partial_target_sum: Vec<Vec<i32>> = vec![];
//             if let Some(Some(found)) = sum_combinations.get(&partial_target) {
//                 // if the sum_combinations already holds the answer, we're good
//                 combinations_with_partial_target_sum = found.clone();
//             } else {
//                 // otherwise, we have to use recursion to fill that in for us
//                 if let Some(found) = recursively_get_combinations_with_target_sum(
//                     partial_target,
//                     attainable_sums,
//                     sum_combinations,
//                 ) {
//                     attainable_sums.push(partial_target);
//                     sum_combinations.insert(partial_target, Some(found.clone()));
//                     combinations_with_partial_target_sum = found
//                 }
//             }
//             for combination in combinations_with_partial_target_sum.into_iter() {
//                 combinations.push(vec![vec![*attainable_sum; n as usize], combination].concat());
//             }
//         }
//     }
//     match combinations.len() {
//         0 => None,
//         _ => Some(combinations),
//     }
// }
