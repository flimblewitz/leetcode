fn main() {
    assert_eq!(
        Solution::subsets_with_dup(vec![1, 2, 2]),
        vec![
            vec![],
            vec![1],
            vec![1, 2],
            vec![1, 2, 2],
            vec![2],
            vec![2, 2]
        ]
    )
}

struct Solution {}
impl Solution {
    pub fn subsets_with_dup(nums: Vec<i32>) -> Vec<Vec<i32>> {
        // just get all possible combinations, then deduplicate them
        use std::collections::HashSet;
        let mut subsets: HashSet<Vec<i32>> = HashSet::new();
        subsets.insert(vec![]);

        for num in nums {
            //// we can clone the whole set, but that's pretty wasteful in terms of memory
            // subsets.clone().into_iter().for_each(|mut v| {
            //// let's just collect the items inside, clone them into a vec, and quickly iterate over it
            let subsets_as_vec: Vec<_> = subsets.iter().cloned().collect();
            subsets_as_vec.into_iter().for_each(|mut v| {
                v.push(num);
                v.sort();
                subsets.insert(v);
            });
        }
        subsets.into_iter().collect()
    }
}
