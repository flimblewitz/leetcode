fn main() {
    println!("{:?}", Solution::subsets(vec![1, 2, 3]));
    println!("{:?}", Solution::subsets(vec![0]));
}
struct Solution {}
impl Solution {
    pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
        // maybe do this "backwards". Start with [[]]
        // then, for each num, duplicate each item with the given num included
        nums.iter().fold(vec![vec![]], |mut acc, &num| {
            acc.append(
                &mut acc
                    .iter()
                    .map(|subset| {
                        let mut s = subset.clone();
                        s.push(num);
                        s
                    })
                    .collect(),
            );
            acc
        })
    }
}
