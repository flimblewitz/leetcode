fn main() {
    // println!("{:?}", Solution::generate_matrix(1));
    // println!("{:?}", Solution::generate_matrix(2));
    println!("{:?}", Solution::generate_matrix(3));
}

struct Solution {}
impl Solution {
    pub fn generate_matrix(n: i32) -> Vec<Vec<i32>> {
        let mut matrix = vec![vec![1; n as usize]; n as usize];
        if n == 1 {
            return matrix;
        }

        let mut left = 0_usize;
        let mut right = n as usize;
        let mut top = 0_usize;
        let mut bottom = n as usize;

        let mut n = 1;
        loop {
            // go right
            for col in left..right {
                matrix[top][col] = n;
                n += 1;
            }
            // let's reign in right to be in bounds and use that
            right -= 1;
            // let's increment top because the top row is now done
            top += 1;
            // if we can't go down, stop
            if top >= bottom {
                break;
            }

            // go down
            for row in (top..bottom) {
                matrix[row][right] = n;
                n += 1;
            }
            // we keep right where it is because we will exclusively iterate toward it on the next left and right movements
            // let's reign in right to be in bounds and use that
            bottom -= 1;
            // if we can't go left, stop
            if left >= right {
                break;
            }

            // go left
            for col in (left..right).rev() {
                matrix[bottom][col] = n;
                n += 1;
            }
            // we keep bottom where it is because it will be exclusive
            // we still haven't covered the leftmost column
            // if we were able to go left, we must also be able to go up still

            // go up
            for row in (top..bottom).rev() {
                matrix[row][left] = n;
                n += 1;
            }
            // let's reign in left because it has been totally used
            left += 1;
            // if we can't go right, stop
            if left >= right {
                break;
            }

            // go up
        }

        matrix
    }
}
