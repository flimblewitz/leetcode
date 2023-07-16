fn main() {
    assert_eq!(Solution::peak_index_in_mountain_array(vec![0, 1, 0]), 1);
    assert_eq!(Solution::peak_index_in_mountain_array(vec![0, 2, 1, 0]), 1);
    assert_eq!(Solution::peak_index_in_mountain_array(vec![0, 1, 2, 0]), 2);
    assert_eq!(Solution::peak_index_in_mountain_array(vec![0, 10, 5, 2]), 1);
    assert_eq!(
        Solution::peak_index_in_mountain_array(vec![24, 69, 100, 99, 79, 78, 67, 36, 26, 19]),
        2
    );
    assert_eq!(
        Solution::peak_index_in_mountain_array(vec![
            1, 3, 29, 30, 34, 35, 42, 60, 64, 73, 91, 94, 91, 85, 80, 75, 71, 63, 54, 53, 42, 27,
            24, 21, 14, 11, 10, 9
        ]),
        11
    );
}
struct Solution;

impl Solution {
    pub fn peak_index_in_mountain_array(arr: Vec<i32>) -> i32 {
        let mut l = 0;
        let mut r = arr.len() - 1;
        // the premise that we will maintain with every iteration is that the subarray is a mountain
        while l + 1 < r {
            let m = (l + r) / 2;
            match (arr[m - 1] < arr[m], arr[m] < arr[m + 1]) {
                (true, true) => l = m + 1,
                (true, false) => return m as i32,
                (false, true) => unreachable!("this would mean that the array is not a mountain"),
                (false, false) => r = m,
            }
        }
        // so when that loop terminates, we're left with l + 1 == r
        if arr[l] < arr[r] {
            r as i32
        } else {
            l as i32
        }
    }
}
