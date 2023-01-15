fn main() {
    // let mut board = vec![
    //     vec!['X', 'X', 'X', 'X'],
    //     vec!['X', 'O', 'O', 'X'],
    //     vec!['X', 'X', 'O', 'X'],
    //     vec!['X', 'O', 'X', 'X'],
    // ];
    // Solution::solve(&mut board);
    // println!("{:?}", board);
    let mut board = vec![
        vec!['X', 'O', 'X', 'O', 'X', 'O'],
        vec!['O', 'X', 'O', 'X', 'O', 'X'],
        vec!['X', 'O', 'X', 'O', 'X', 'O'],
        vec!['O', 'X', 'O', 'X', 'O', 'X'],
    ];
    println!("{:?}", board);
    Solution::solve(&mut board);
    println!("{:?}", board);
    // let mut board = vec![
    //     vec!['O', 'X', 'X', 'O', 'X'],
    //     vec!['X', 'O', 'O', 'X', 'O'],
    //     vec!['X', 'O', 'X', 'O', 'X'],
    //     vec!['O', 'X', 'O', 'O', 'O'],
    //     vec!['X', 'X', 'O', 'X', 'O'],
    // ];
    // println!("{:?}", board);
    // Solution::solve(&mut board);
    // println!("{:?}", board);
}

struct Solution;

use std::collections::HashSet;

type Coord = (usize, usize);

impl Solution {
    pub fn solve(board: &mut Vec<Vec<char>>) {
        // let's get a list of all edge Os
        // for each edge O, perform dfs and flip to X
        let (first_row_coords, last_row_coords): (Vec<Coord>, Vec<Coord>) = (0..board[0].len())
            .map(|col| ((0_usize, col), (board.len() - 1, col)))
            .unzip();
        // this excludes the first and last coords beause they're already counted in the other two above
        let (first_col_coords, last_col_coords): (Vec<Coord>, Vec<Coord>) = (1..board.len() - 1)
            .map(|row| ((row, 0_usize), (row, board[0].len() - 1)))
            .unzip();
        // println!(
        //     "{:?}, {:?}, {:?}, {:?}",
        //     first_row_coords, last_row_coords, first_col_coords, last_col_coords
        // );
        let edge_o_coords: Vec<Coord> = [
            first_row_coords,
            last_row_coords,
            first_col_coords,
            last_col_coords,
        ]
        .into_iter()
        .flatten()
        .filter(|(row, col)| board[*row][*col] == 'O')
        // .cloned() // for some reason leetcode claims that the iterator is iterating over (&usize, &usize) items and needs this? It must be a rust version difference
        .collect();
        // println!("{:?}", edge_o_coords);

        // dfs to identify edge regions
        let mut traversed_coords: HashSet<Coord> = HashSet::new();
        let mut edge_region_coords: HashSet<Coord> = HashSet::new();
        edge_o_coords.into_iter().for_each(|coord| {
            recursively_identify_edge_region_coords(
                board,
                coord,
                &mut traversed_coords,
                &mut edge_region_coords,
                0,
            )
        });

        // flip all Os not in edge regions to Xs
        board.iter_mut().enumerate().for_each(|(row_index, row)| {
            row.iter_mut().enumerate().for_each(|(col_index, col)| {
                if !edge_region_coords.contains(&(row_index, col_index)) {
                    *col = 'X'
                }
            })
        });
    }
}

fn recursively_identify_edge_region_coords(
    board: &[Vec<char>],
    starting_coord: Coord,
    traversed_coords: &mut HashSet<Coord>,
    edge_region_coords: &mut HashSet<Coord>,
    _depth: usize,
) {
    let (row, col) = starting_coord;
    if traversed_coords.contains(&starting_coord) {
        return;
    }
    // println!(
    //     "{}looking at {:?}",
    //     (0.._depth).map(|_| " ").collect::<String>(),
    //     starting_coord
    // );
    traversed_coords.insert(starting_coord);
    if board[row][col] == 'X' {
        return;
    }
    edge_region_coords.insert(starting_coord);
    if row > 0 {
        recursively_identify_edge_region_coords(
            board,
            (row - 1, col),
            traversed_coords,
            edge_region_coords,
            _depth + 1,
        )
    }
    if row < board.len() - 1 {
        recursively_identify_edge_region_coords(
            board,
            (row + 1, col),
            traversed_coords,
            edge_region_coords,
            _depth + 1,
        )
    }
    if col > 0 {
        recursively_identify_edge_region_coords(
            board,
            (row, col - 1),
            traversed_coords,
            edge_region_coords,
            _depth + 1,
        )
    }
    if col < board[0].len() - 1 {
        recursively_identify_edge_region_coords(
            board,
            (row, col + 1),
            traversed_coords,
            edge_region_coords,
            _depth + 1,
        )
    }
}

// pub fn solve(board: &mut Vec<Vec<char>>) {
//     (1..board.len() - 1).for_each(|row| {
//         (1..board[row].len() - 1).for_each(|col| {
//             println!("{}", board[row][col]);
//             if board[row][col] == 'O' && is_surrounded_by_x(row, col, board) {
//                 board[row][col] = 'X'
//             }
//         })
//     });
// }
// // assume non-edge row/col
// fn is_surrounded_by_x(row: usize, col: usize, board: &Vec<Vec<char>>) -> bool {
//     println!("{row}, {col}");
//     let has_x_to_left = board[row][..col].iter().any(is_x);
//     let has_x_to_right = board[row][col + 1..].iter().any(is_x);
//     let has_x_above = (0..row).map(|r| &board[r][col]).any(is_x);
//     let has_x_below = (row + 1..board.len()).map(|r| &board[r][col]).any(is_x);
//     has_x_to_left && has_x_to_right && has_x_above && has_x_below
// }

// fn is_x(c: &char) -> bool {
//     *c == 'X'
// }
