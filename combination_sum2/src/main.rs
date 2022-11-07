fn main() {
    // println!("{:?}", Solution::combination_sum2(vec![2, 3, 6, 7], 7));
    // println!(
    //     "{:?}",
    //     Solution::combination_sum2(vec![10, 1, 2, 7, 6, 1, 5], 8)
    // );
    // println!("{:?}", Solution::combination_sum2(vec![2, 5, 2, 1, 2], 5));
    println!(
        "{:?}",
        Solution::combination_sum2(
            vec![
                29, 19, 14, 33, 11, 5, 9, 23, 23, 33, 12, 9, 25, 25, 12, 21, 14, 11, 20, 30, 17,
                19, 5, 6, 6, 5, 5, 11, 12, 25, 31, 28, 31, 33, 27, 7, 33, 31, 17, 13, 21, 24, 17,
                12, 6, 16, 20, 16, 22, 5
            ],
            28
        )
    );
    /*
    [[7, 6, 5, 5, 5], [13, 5, 5, 5], [6, 6, 6, 5, 5], [12, 6, 5, 5], [11, 7, 5, 5], [9, 9, 5, 5], [11, 6, 6, 5], [17, 6, 5], [16, 7, 5], [14, 9, 5], [12, 11, 5], [23, 5], [9, 7, 6, 6], [16, 6, 6], [13, 9, 6], [11, 11, 6], [22, 6], [12, 9, 7], [21, 7], [19, 9], [17, 11], [16, 12], [14, 14], [28]]
    */
}

struct Solution {}
impl Solution {
    pub fn combination_sum2(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
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
    let mut i = index;
    while i < candidates.len() {
        let c = candidates[i];

        if c > target {
            break;
        }

        let mut partial_combinations = recurse(candidates, target - c, i + 1);
        if partial_combinations.len() > 0 {
            for pc in partial_combinations.iter_mut() {
                pc.append(&mut vec![c; 1]);
            }
            v.append(&mut partial_combinations);
        }

        if c == target {
            v.push(vec![c; 1]);
        }

        // increment, but keep incrementing if the number hasn't changed
        i += 1;
        while i < candidates.len() && candidates[i - 1] == candidates[i] {
            i += 1;
        }
    }
    v
}
