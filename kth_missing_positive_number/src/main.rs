fn main() {
    assert_eq!(Solution::find_kth_positive(vec![2, 3, 4, 7, 11], 5), 9);
    println!();
    assert_eq!(Solution::find_kth_positive(vec![2, 3, 4, 7, 11], 7), 12);
    println!();
    assert_eq!(Solution::find_kth_positive(vec![1, 2, 3, 4], 2), 6);

    assert_eq!(Solution::find_kth_positive_2(vec![2, 3, 4, 7, 11], 5), 9);
    println!();
    assert_eq!(Solution::find_kth_positive_2(vec![2, 3, 4, 7, 11], 7), 12);
    println!();
    assert_eq!(Solution::find_kth_positive_2(vec![1, 2, 3, 4], 2), 6);
}
struct Solution;

impl Solution {
    pub fn find_kth_positive(arr: Vec<i32>, k: i32) -> i32 {
        let mut i = 0;
        let mut expected_ith_positive = 1;
        let mut num_missing_positives = 0;
        while i < arr.len() {
            // println!("expected_ith_positive: {expected_ith_positive}");
            // println!("num_missing_positives: {num_missing_positives}");
            if arr[i] == expected_ith_positive {
                expected_ith_positive += 1;
                i += 1;
            } else {
                let num_new_missing_positives = arr[i] - expected_ith_positive;
                // println!("num_new_missing_positives: {num_new_missing_positives}");
                if num_missing_positives + num_new_missing_positives >= k {
                    // println!("returning early");
                    return (expected_ith_positive - 1) + (k - num_missing_positives);
                }
                expected_ith_positive += num_new_missing_positives;
                num_missing_positives += num_new_missing_positives;
                // println!("ending iteration with num_missing_positives {num_missing_positives}");
            }
        }
        arr.last().unwrap() + (k - num_missing_positives)
    }

    // the other solution works fine, but I wonder if reducing k to 0 instead of using a "num_missing_positives" variable makes it easier to read
    pub fn find_kth_positive_2(arr: Vec<i32>, mut k: i32) -> i32 {
        let mut i = 0;
        let mut expected_ith_positive = 1;
        while i < arr.len() {
            if arr[i] == expected_ith_positive {
                expected_ith_positive += 1;
                i += 1;
            } else {
                let num_new_missing_positives = arr[i] - expected_ith_positive;
                if num_new_missing_positives >= k {
                    return (expected_ith_positive - 1) + k;
                }
                expected_ith_positive += num_new_missing_positives;
                k -= num_new_missing_positives;
            }
        }
        arr.last().unwrap() + k
    }
}
