fn main() {
    println!("{:?}", Solution::total_n_queens(1));
    println!("{:?}", Solution::total_n_queens(2));
    println!("{:?}", Solution::total_n_queens(3));
    println!("{:?}", Solution::total_n_queens(4));
    println!("{:?}", Solution::total_n_queens(5));
}

struct Solution {}
impl Solution {
    pub fn total_n_queens(n: i32) -> i32 {
        let initial_board = vec![(0..n).collect(); n as usize];
        recursively_get_num_solutions(initial_board) as i32
    }
}

fn recursively_get_num_solutions(board: Vec<Vec<i32>>) -> usize {
    if board.len() == 1 {
        return board[0].len();
    }
    let mut num_solutions = 0;

    for &c in board[0].iter() {
        let mut sub_board = board.clone();
        let mut i = 1;
        for row in sub_board.iter_mut().skip(1) {
            let left_column_index_to_delete = c - i;
            let right_column_index_to_delete = c + i;
            *row = row
                .into_iter()
                .filter_map(|row_c| {
                    match *row_c != c
                        && *row_c != left_column_index_to_delete
                        && *row_c != right_column_index_to_delete
                    {
                        true => Some(*row_c),
                        _ => None,
                    }
                })
                .collect();
            i += 1;
        }
        num_solutions += recursively_get_num_solutions(sub_board.iter().skip(1).cloned().collect())
    }
    num_solutions
}
