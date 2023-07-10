fn main() {
    assert_eq!(
        Solution::search_matrix(
            vec![vec![1, 3, 5, 7], vec![10, 11, 16, 20], vec![23, 30, 34, 60]],
            3
        ),
        true
    );
    assert_eq!(Solution::search_matrix(vec![vec![1, 1]], 3), false);
    assert_eq!(Solution::search_matrix(vec![vec![1, 3]], 3), true);
}

struct Solution;

impl Solution {
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        let rows = matrix.len();
        let cols = matrix[0].len();
        let mut l = 0;
        let mut r = rows * cols - 1;
        while l < r {
            // println!("l {l} r {r}");
            let m = (l + r) / 2;
            if target <= matrix[m / cols][m % cols] {
                r = m;
            } else {
                l = m + 1;
            }
        }
        matrix[r / cols][r % cols] == target
    }
}
