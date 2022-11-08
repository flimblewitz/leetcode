fn main() {
    println!("{:?}", Solution::permute(vec![1, 2, 3]));
}

struct Solution {}
impl Solution {
    pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
        get_permutations_recursively(nums)
    }
}

fn get_permutations_recursively(nums: Vec<i32>) -> Vec<Vec<i32>> {
    if nums.len() == 1 {
        return vec![nums];
    }

    let mut permutations = vec![];

    for i in 0..nums.len() {
        let mut nums_variant = nums.clone();
        let tmp = nums_variant[0];
        nums_variant[0] = nums_variant[i];
        nums_variant[i] = tmp;
        let sub_permutations =
            get_permutations_recursively(nums_variant.iter().skip(1).cloned().collect());
        for sp in sub_permutations {
            permutations.push(vec![vec![nums_variant[0]], sp].concat());
        }
    }

    permutations
}
