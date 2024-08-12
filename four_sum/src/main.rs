fn main() {
    assert_eq!(
        Solution::four_sum(vec![1, 0, -1, 0, -2, 2], 0),
        vec![vec![-2, -1, 1, 2], vec![-2, 0, 0, 2], vec![-1, 0, 0, 1]]
    );
    assert_eq!(
        Solution::four_sum(vec![2, 2, 2, 2, 2], 8),
        vec![[2, 2, 2, 2]]
    );
}

struct Solution;

use std::collections::{HashMap, HashSet};
impl Solution {
    // note that sorting the return results is unnecessary for the leetcode submission, but it means the test cases can be run locally

    // this is fast _enough_ but still wasteful. The right memoization strategy is a little tough to identify here
    pub fn four_sum(mut nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        if nums.len() < 4 {
            return vec![];
        }
        nums.sort_unstable();
        let mut solutions: HashSet<(i32, i32, i32, i32)> = HashSet::new();
        for w in 0..nums.len() - 3 {
            if w > 0 && nums[w - 1] == nums[w] {
                continue;
            }
            for x in w + 1..nums.len() - 2 {
                if x - 1 > w && nums[x - 1] == nums[x] {
                    continue;
                }
                for y in x + 1..nums.len() - 1 {
                    if y - 1 > x && nums[y - 1] == nums[y] {
                        continue;
                    }
                    for z in y + 1..nums.len() {
                        // println!("{}, {}, {}, {}", nums[w], nums[x], nums[y], nums[z]);
                        if [w, x, y, z]
                            .into_iter()
                            .try_fold(0_i32, |acc, i| acc.checked_add(nums[i]))
                            .is_some_and(|sum| sum == target)
                        {
                            solutions.insert((nums[w], nums[x], nums[y], nums[z]));
                            break;
                        }
                    }
                }
            }
        }
        let mut solutions: Vec<Vec<i32>> = solutions
            .into_iter()
            .map(|(a, b, c, d)| vec![a, b, c, d])
            .collect();
        solutions.sort_unstable();
        solutions
    }

    // here's the naive solution. It takes too long
    pub fn alternative_1(mut nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        nums.sort_unstable();
        let mut solutions: HashSet<(i32, i32, i32, i32)> = HashSet::new();
        for w in 0..nums.len() - 3 {
            for x in w + 1..nums.len() - 2 {
                for y in x + 1..nums.len() - 1 {
                    for z in y + 1..nums.len() {
                        // println!("{}, {}, {}, {}", nums[w], nums[x], nums[y], nums[z]);
                        if nums[w] + nums[x] + nums[y] + nums[z] == target {
                            // solutions.push(vec![nums[w], nums[x], nums[y], nums[z]]);
                            solutions.insert((nums[w], nums[x], nums[y], nums[z]));
                        }
                    }
                }
            }
        }
        let mut solutions: Vec<Vec<i32>> = solutions
            .into_iter()
            .map(|(a, b, c, d)| vec![a, b, c, d])
            .collect();
        solutions.sort_unstable();
        solutions
    }
}
