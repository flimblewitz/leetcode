fn main() {
    println!(
        "{:?}",
        Solution::is_valid_sudoku(vec![
            vec!['5', '3', '.', '.', '7', '.', '.', '.', '.'],
            vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
            vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
            vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
            vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
            vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
            vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
            vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
            vec!['.', '.', '.', '.', '8', '.', '.', '7', '9']
        ])
    );
    println!(
        "{:?}",
        Solution::is_valid_sudoku(vec![
            vec!['8', '3', '.', '.', '7', '.', '.', '.', '.'],
            vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
            vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
            vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
            vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
            vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
            vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
            vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
            vec!['.', '.', '.', '.', '8', '.', '.', '7', '9']
        ])
    );
}

struct Solution {}
impl Solution {
    // rayon would be great for this, but whatever
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        // for each row, confirm uniqueness
        for r in 0..9 {
            let mut used_digits = [false; 10];
            for c in 0..9 {
                let digit = board[r][c].to_digit(10);
                if let Some(d) = digit {
                    if used_digits[d as usize] {
                        return false;
                    }
                    used_digits[d as usize] = true;
                }
            }
        }
        // for each column, confirm uniqueness
        for c in 0..9 {
            let mut used_digits = [false; 10];
            for r in 0..9 {
                let digit = board[r][c].to_digit(10);
                if let Some(d) = digit {
                    if used_digits[d as usize] {
                        return false;
                    }
                    used_digits[d as usize] = true;
                }
            }
        }

        // for each block, confirm uniqueness
        let indexes: Vec<u32> = (0_u32..9_u32).collect();
        for block_row_chunks in indexes.chunks(3) {
            for block_column_chunks in indexes.chunks(3) {
                let mut used_digits = [false; 10];
                for r in block_row_chunks {
                    for c in block_column_chunks {
                        let digit = board[*r as usize][*c as usize].to_digit(10);
                        if let Some(d) = digit {
                            if used_digits[d as usize] {
                                return false;
                            }
                            used_digits[d as usize] = true;
                        }
                    }
                }
            }
        }

        // otherwise we're good
        true
    }
}
