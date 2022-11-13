fn main() {
    // println!(
    //     "{:?}",
    //     Solution::min_path_sum(vec![vec![1, 3, 1], vec![1, 5, 1], vec![4, 2, 1]])
    // );
    println!(
        "{:?}",
        Solution::min_path_sum(vec![vec![1, 2, 3], vec![4, 5, 6]])
    );
}
struct Solution {}
use std::collections::HashMap;
impl Solution {
    pub fn min_path_sum(grid: Vec<Vec<i32>>) -> i32 {
        // memoize by coordinate
        let mut sub_solutions: HashMap<(usize, usize), i32> = HashMap::new();
        recursively_get_min_path_sum(&mut sub_solutions, &grid, 0, 0)
    }
}
fn recursively_get_min_path_sum(
    sub_solutions: &mut HashMap<(usize, usize), i32>,
    grid: &Vec<Vec<i32>>,
    r: usize,
    c: usize,
) -> i32 {
    if let Some(&solution) = sub_solutions.get(&(r, c)) {
        return solution;
    }
    if r == grid.len() - 1 && c == grid[0].len() - 1 {
        return grid[r][c];
    }
    let down_path_sum = match r < grid.len() - 1 {
        true => recursively_get_min_path_sum(sub_solutions, grid, r + 1, c),
        false => std::i32::MAX,
    };
    let right_path_sum = match c < grid[0].len() - 1 {
        true => recursively_get_min_path_sum(sub_solutions, grid, r, c + 1),
        false => std::i32::MAX,
    };
    let min_path_sum = grid[r][c] + down_path_sum.min(right_path_sum);
    sub_solutions.insert((r, c), min_path_sum);
    min_path_sum
}
