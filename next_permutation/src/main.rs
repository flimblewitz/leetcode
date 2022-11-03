fn main() {
    // let mut v = vec![1];
    let mut v = vec![1, 2, 3];
    // let mut v = vec![3, 2, 1];
    // let mut v = vec![2, 3, 1];
    // let mut v = vec![4, 2, 0, 2, 3, 2, 0];
    // should be [4,2,0,3,0,2,2]
    // I produce [4,2,2,0,0,2,3]
    Solution::next_permutation(&mut v);
    println!("{:?}", v);
}

struct Solution {}
impl Solution {
    pub fn next_permutation(nums: &mut Vec<i32>) {
        let len = nums.len();
        let mut is_last_permutation = true;
        'outer: for left_outer in (0..len).rev() {
            // for right in (0..len).rev() {
            for right in (0..len).rev() {
                for left in (left_outer..right).rev() {
                    if nums[right] > nums[left] {
                        is_last_permutation = false;
                        let tmp = nums[left];
                        nums[left] = nums[right];
                        nums[right] = tmp;
                        nums[(left + 1)..len].sort();
                        break 'outer;
                    }
                }
            }
        }
        if is_last_permutation {
            nums.sort();
        }
    }
}
