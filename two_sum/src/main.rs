fn main() {
    assert_eq!(Solution::two_sum(vec![2, 7, 11, 15], 9), vec![0, 1]);
    assert_eq!(Solution::two_sum(vec![3, 2, 4], 6), vec![1, 2]);
    assert_eq!(Solution::two_sum(vec![3, 3], 6), vec![0, 1]);
}

struct Solution;

use std::collections::{HashMap, HashSet};
impl Solution {
    // memoization is overkill for an easy problem beause it has a forgiving test input, but optimization is fun regardless
    // this was neat because its a reminder that, as is usually the case, it's possible to take your initial naive idea for how to solve a problem and turn it inside out to cook up a satisfyingly superior solution
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        // in previous attempts, I thought the natural way to memoize information was to record the _losing_ numbers. However, it's way more efficient here to memoize potential _winning_ numbers. This is because when you memoize losing numbers, it's O(n^2) because you're comparing all possible PAIRS, and furthemore you only get to memoize one thing after each n iterations of the n^2 iterations. But If you memoize possible winning numbers, you can keep it O(n) because on each iteration you can either immediately tell if it's a winner or identify another potential winner for the subsequent iterations
        let mut potential_winners: HashMap<i32, i32> = HashMap::with_capacity(nums.len());
        nums.iter()
            .enumerate()
            .find_map(|(i, &num)| {
                potential_winners
                    .get(&num)
                    .and_then(|&j| Some(vec![i.try_into().unwrap(), j]))
                    .or_else(|| {
                        // we could just use .insert() and possibly insert a duplicate value, but it kinda rubs me the wrong way and I expect it's less efficient, albeit probably negligibly
                        potential_winners
                            .entry(target - num)
                            .or_insert(i.try_into().unwrap());
                        None
                    })
                // this is just an alternative and equivalent expression. I don't know which I prefer
                // match potential_winners.get(&num) {
                //     Some(&j) => Some(vec![i.try_into().unwrap(), j]),
                //     None => {
                //         potential_winners
                //             .entry(target - num)
                //             .or_insert(i.try_into().unwrap());
                //         None
                //     }
                // }
            })
            .unwrap()
    }

    pub fn alternative_3(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut useless_numbers: HashSet<i32> = HashSet::with_capacity(nums.len());
        nums.iter()
            .enumerate()
            .take(nums.len() - 1)
            .find_map(|(i, &first)| {
                if useless_numbers.contains(&first) {
                    return None;
                };
                let desired = target - first;
                nums.get(i + 1..).and_then(|remaining_nums| {
                    remaining_nums
                        .iter()
                        .enumerate()
                        .map(|(j, val)| (i + 1 + j, val))
                        .find_map(|(j, &second)| {
                            if second == desired {
                                Some(vec![i.try_into().unwrap(), j.try_into().unwrap()])
                            } else {
                                None
                            }
                        })
                        .or_else(|| {
                            // if we found no matches, let's memoize that the first number is useless so we can ignore any duplicates later
                            useless_numbers.insert(first);
                            None
                        })
                })
            })
            .unwrap()
    }

    pub fn alternative_2(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut useless_numbers: HashSet<i32> = HashSet::with_capacity(nums.len());
        nums.iter()
            .enumerate()
            .take(nums.len() - 1)
            .find_map(|(i, &first)| {
                if useless_numbers.contains(&first) {
                    return None;
                };
                let desired = target - first;
                //
                nums.iter()
                    .enumerate()
                    .skip(i + 1)
                    .find_map(|(j, &second)| {
                        if second == desired {
                            Some(vec![i.try_into().unwrap(), j.try_into().unwrap()])
                        } else {
                            None
                        }
                    })
                    .or_else(|| {
                        useless_numbers.insert(first);
                        None
                    })
            })
            .unwrap()
    }

    // here's a variation that doesn't use iterators as much. While it's fewer lines, interestingly, it seems to perform worse
    pub fn alternative_1(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut useless_numbers: HashSet<i32> = HashSet::with_capacity(nums.len());
        for i in 0..(nums.len() - 1) {
            if useless_numbers.contains(&nums[i]) {
                continue;
            }
            if let Some(j) = (i + 1..nums.len()).find(|&j| nums[i] + nums[j] == target) {
                return vec![i.try_into().unwrap(), j.try_into().unwrap()];
            }
            useless_numbers.insert(nums[i]);
        }
        panic!("oops")
    }
}
