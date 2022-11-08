fn main() {
    // let mut v = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
    // Solution::rotate(&mut v);
    // println!("{:?}", v);
    let mut v = vec![
        vec![5, 1, 9, 11],
        vec![2, 4, 8, 10],
        vec![13, 3, 6, 7],
        vec![15, 14, 12, 16],
    ];
    Solution::rotate(&mut v);
    println!("{:?}", v);
}

struct Solution {}
impl Solution {
    pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
        let n = matrix.len();

        for ring_index in 0..(n / 2) {
            let last_index_in_ring = n - ring_index - 1;
            for m in 0..(last_index_in_ring - ring_index) {
                // get top left
                let tmp = matrix[ring_index][ring_index + m];
                // put bottom left in top left
                matrix[ring_index][ring_index + m] = matrix[last_index_in_ring - m][ring_index];
                // put bottom right in bottom left
                matrix[last_index_in_ring - m][ring_index] =
                    matrix[last_index_in_ring][last_index_in_ring - m];
                // put top right in bottom right
                matrix[last_index_in_ring][last_index_in_ring - m] =
                    matrix[ring_index + m][last_index_in_ring];
                // put top left in top right
                matrix[ring_index + m][last_index_in_ring] = tmp;
            }
        }
    }
}
