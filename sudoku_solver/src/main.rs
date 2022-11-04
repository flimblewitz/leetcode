fn main() {
    let mut v = vec![
        vec!['5', '3', '.', '.', '7', '.', '.', '.', '.'],
        vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
        vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
        vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
        vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
        vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
        vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
        vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
        vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
    ];
    Solution::solve_sudoku(&mut v);
    println!("{:?}", v);
}

struct Solution {}
impl Solution {
    pub fn solve_sudoku(board: &mut Vec<Vec<char>>) {
        let _ = recursively_solve(board, 0);
    }
}

fn recursively_solve(board: &mut Vec<Vec<char>>, cell_number: usize) -> Result<(), ()> {
    // for r in 0..9_usize {
    //     for c in 0..9_usize {
    for cn in cell_number..81 {
        // if cn == 78 {
        //     println!("hmmm");
        // }
        let r = cn / 9;
        let c = cn % 9;
        if board[r][c] == '.' {
            let possible_values = get_possible_values(board, r, c);
            // println!("possible_values for {cn}, AKA {r},{c}: {possible_values:?}");
            // println!("current state:");
            // println!("{board:?}");
            for possible_value in possible_values {
                // println!("mutating {cn}, AKA {r},{c} to {possible_value}");
                board[r][c] = possible_value;
                if let Ok(_) = recursively_solve(board, cn + 1) {
                    return Ok(());
                }
                // we might be done!
                if board
                    .iter()
                    .all(|row| row.iter().all(|value| value.is_digit(10)))
                {
                    return Ok(());
                }
                board[r][c] = '.';
            }
            // if none of the possible values were acceptable, back out
            if board[r][c] == '.' {
                return Err(());
            }
        }
    }
    Err(())
}

fn get_possible_values(board: &mut Vec<Vec<char>>, cell_row: usize, cell_col: usize) -> Vec<char> {
    let mut used = vec![false; 9];
    // iterate over the row
    for c in 0..9_usize {
        if let Some(val) = board[cell_row][c].to_digit(10) {
            used[val as usize - 1] = true;
        }
    }
    // iterate over the column
    for r in 0..9_usize {
        if let Some(val) = board[r][cell_col].to_digit(10) {
            used[val as usize - 1] = true;
        }
    }
    // iterate over the block
    let block_row = cell_row / 3;
    let first_row = block_row * 3;
    let block_col = cell_col / 3;
    let first_col = block_col * 3;
    for r in first_row..(first_row + 3) {
        for c in first_col..(first_col + 3) {
            if let Some(val) = board[r][c].to_digit(10) {
                used[val as usize - 1] = true;
            }
        }
    }
    // return the values that haven't been used
    used.iter()
        .enumerate()
        .filter(|(_, is_used)| !**is_used)
        .map(|(index, _)| std::char::from_digit(index as u32 + 1, 10).unwrap())
        .collect()
}
