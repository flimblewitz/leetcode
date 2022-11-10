fn main() {
    // println!(
    //     "{:?}",
    //     Solution::spiral_order(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]])
    // );
    // println!(
    //     "{:?}",
    //     Solution::spiral_order(vec![
    //         vec![1, 2, 3, 4],
    //         vec![5, 6, 7, 8],
    //         vec![9, 10, 11, 12]
    //     ])
    // );
    // println!("{:?}", Solution::spiral_order(vec![vec![3], vec![2],]));
    // println!(
    //     "{:?}",
    //     Solution::spiral_order(vec![vec![7], vec![9], vec![6]])
    // );
    println!(
        "{:?}",
        Solution::spiral_order(vec![vec![2, 5, 8], vec![4, 0, -1]])
    );
}

struct Solution {}
impl Solution {
    pub fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
        let mut top = 0;
        let mut right = matrix[0].len();
        let mut bottom = matrix.len();
        let mut left = 0;
        let mut v = vec![];
        loop {
            matrix[top][left..right].iter().for_each(|&num| v.push(num));
            // println!("{:?}", v);
            top += 1;
            if top > bottom {
                break;
            }
            right -= 1;
            if left > right {
                break;
            }

            matrix[top..bottom]
                .iter()
                .map(|row| row[right])
                .for_each(|num| v.push(num));
            // println!("{:?}", v);
            bottom -= 1;
            if top > bottom {
                break;
            }
            // if left and right have converged, it's time to stop
            if left >= right {
                break;
            }

            matrix[bottom][left..right]
                .iter()
                .rev()
                .for_each(|&num| v.push(num));

            // if top and bottom have converged, it's time to stop
            if top >= bottom {
                break;
            }

            matrix[top..bottom]
                .iter()
                .rev()
                .map(|row| row[left])
                .for_each(|num| v.push(num));
            // println!("{:?}", v);
            left += 1;
            if left > right {
                break;
            }
        }

        v
    }
}
