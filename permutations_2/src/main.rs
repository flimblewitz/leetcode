fn main() {
    // println!("{:?}", Solution::permute_unique(vec![1, 2, 3]));
    // println!("{:?}", Solution::permute_unique(vec![1, 2, 1]));

    let v = vec![2, 2, 1, 1];
    println!("{:?}", &v[4..]);

    println!("{:?}", Solution::permute_unique(vec![2, 2, 1, 1])); // [[2, 2, 1, 1], [2, 1, 2, 1], [2, 1, 1, 2], [1, 2, 2, 1], [1, 2, 1, 2], [1, 1, 2, 2]]
    println!("{:?}", Solution::permute_unique(vec![3, 3, 0, 3])); // [[0,3,3,3],[3,0,3,3],[3,3,0,3],[3,3,3,0]]
    println!("{:?}", Solution::permute_unique(vec![0, 1, 0, 0, 9]));
    // good
    // [[0,0,0,1,9],[0,0,0,9,1],[0,0,1,0,9],[0,0,1,9,0],[0,0,9,0,1],[0,0,9,1,0],[0,1,0,0,9],[0,1,0,9,0],[0,1,9,0,0],[0,9,0,0,1],[0,9,0,1,0],[0,9,1,0,0],[1,0,0,0,9],[1,0,0,9,0],[1,0,9,0,0],[1,9,0,0,0],[9,0,0,0,1],[9,0,0,1,0],[9,0,1,0,0],[9,1,0,0,0]]
    // bad
    // [[0,0,0,1,9],[0,0,0,9,1],[0,0,1,0,9],[0,0,1,9,0],[0,0,9,1,0],[0,0,9,0,1],[0,1,0,0,9],[0,1,0,9,0],[0,1,9,0,0],[0,9,0,1,0],[0,9,0,0,1],[0,9,1,0,0],[0,9,0,1,0],[0,9,0,0,1],[1,0,0,0,9],[1,0,0,9,0],[1,0,9,0,0],[1,9,0,0,0],[9,0,0,1,0],[9,0,0,0,1],[9,0,1,0,0],[9,0,0,1,0],[9,0,0,0,1],[9,1,0,0,0],[9,0,0,1,0],[9,0,0,0,1],[9,0,1,0,0],[9,0,0,1,0],[9,0,0,0,1]]
    // [[0, 0, 0, 1, 9], [0, 0, 0, 9, 1], [0, 0, 1, 0, 9], [0, 0, 1, 9, 0], [0, 0, 9, 1, 0], [0, 0, 9, 0, 1], [0, 1, 0, 0, 9], [0, 1, 0, 9, 0], [0, 1, 9, 0, 0], [0, 9, 0, 1, 0], [0, 9, 0, 0, 1], [0, 9, 1, 0, 0], [0, 9, 0, 1, 0], [0, 9, 0, 0, 1], [1, 0, 0, 0, 9], [1, 0, 0, 9, 0], [1, 0, 9, 0, 0], [1, 9, 0, 0, 0], [9, 0, 0, 1, 0], [9, 0, 0, 0, 1], [9, 0, 1, 0, 0], [9, 0, 0, 1, 0], [9, 0, 0, 0, 1], [9, 1, 0, 0, 0], [9, 0, 0, 1, 0], [9, 0, 0, 0, 1], [9, 0, 1, 0, 0], [9, 0, 0, 1, 0], [9, 0, 0, 0, 1]]
}

struct Solution {}
impl Solution {
    pub fn permute_unique(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        // this is faster but it can swap "equal" elements
        nums.sort_unstable();
        // nums.sort();
        recursive_permute_unique(nums)
    }
}

// this uses some good tricks
fn recursive_permute_unique(nums: Vec<i32>) -> Vec<Vec<i32>> {
    if nums.is_empty() {
        vec![vec![]]
    } else {
        nums.iter()
            .enumerate()
            .fold(
                (None, Vec::new()),
                |(previous, mut permutations), (i, &num)| match previous {
                    Some(j) if j == num => (previous, permutations),
                    _ => {
                        let (left, right) = nums.split_at(i);
                        permutations.append(
                            &mut recursive_permute_unique([left, &right[1..]].concat())
                                .into_iter()
                                .map(|perm| [perm, vec![num]].concat())
                                .collect(),
                        );
                        (Some(num), permutations)
                    }
                },
            )
            .1
    }
}

// impl Solution {
//     pub fn permute_unique(nums: Vec<i32>) -> Vec<Vec<i32>> {
//         // let mut nums = nums.clone();
//         // nums.sort();
//         use std::collections::HashSet;
//         let mut set: HashSet<Vec<i32>> = HashSet::new();
//         // for p in get_unique_permutations_recursively(nums) {
//         for p in get_permutations_recursively(nums) {
//             set.insert(p);
//         }
//         set.into_iter().collect()
//     }
// }

// fn get_permutations_recursively(nums: Vec<i32>) -> Vec<Vec<i32>> {
//     if nums.len() == 1 {
//         return vec![nums];
//     }

//     let mut permutations = vec![];

//     for i in 0..nums.len() {
//         let mut nums_variant = nums.clone();
//         let tmp = nums_variant[0];
//         nums_variant[0] = nums_variant[i];
//         nums_variant[i] = tmp;
//         let sub_permutations =
//             get_permutations_recursively(nums_variant.iter().skip(1).cloned().collect());
//         for sp in sub_permutations {
//             permutations.push(vec![vec![nums_variant[0]], sp].concat());
//         }
//     }

//     permutations
// }

//// I flubbed this one

// fn get_unique_permutations_recursively(nums: Vec<i32>) -> Vec<Vec<i32>> {
//     if nums.len() == 1 {
//         return vec![nums];
//     }

//     let mut permutations = vec![];

//     let mut i = 0;
//     while i < nums.len() {
//         let mut nums_variant = nums.clone();
//         while i + 1 < nums.len() && nums[i] == nums[i + 1] {
//             i += 1;
//         }

//         let tmp = nums_variant[0];
//         nums_variant[0] = nums_variant[i];
//         nums_variant[i] = tmp;

//         let sub_permutations =
//             get_unique_permutations_recursively(nums_variant.iter().skip(1).cloned().collect());

//         for sp in sub_permutations {
//             permutations.push(vec![vec![nums_variant[0]], sp].concat());
//         }

//         i += 1;
//         // while i < nums.len() && nums[i - 1] == nums[i] {
//         //     i += 1;
//         // }
//     }

//     permutations
// }
