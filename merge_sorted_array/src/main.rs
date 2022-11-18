fn main() {
    let mut nums1 = vec![1, 2, 3, 0, 0, 0];
    Solution::merge(&mut nums1, 3, &mut vec![2, 5, 6], 3);
    println!("{:?}", nums1);
    let mut nums1 = vec![4, 0, 0, 0, 0, 0];
    Solution::merge(&mut nums1, 1, &mut vec![1, 2, 3, 5, 6], 5);
    println!("{:?}", nums1);
}
struct Solution {}
impl Solution {
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        *nums1 = nums1.iter().take(m as usize).cloned().collect();
        let mut i = 0_usize;
        let mut j = 0_usize;
        while i < nums1.len() as usize && j < n as usize {
            if nums1[i] <= nums2[j] {
                i += 1;
            } else {
                nums1.insert(i, nums2[j]);
                j += 1;
                i += 1;
            }
        }
        while j < n as usize {
            nums1.push(nums2[j]);
            j += 1;
        }
    }
}
