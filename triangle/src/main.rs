fn main() {
    println!(
        "{}",
        Solution::minimum_total(vec![vec![2], vec![3, 4], vec![6, 5, 7], vec![4, 1, 8, 3]])
    );
    println!("{}", Solution::minimum_total(vec![vec![-10]]));
}

struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn minimum_total(triangle: Vec<Vec<i32>>) -> i32 {
        let mut sub_solutions: HashMap<(usize, usize), i32> = HashMap::new();
        recursively_solve(0, 0, &triangle, &mut sub_solutions)
    }
}

fn recursively_solve(
    row: usize,
    col: usize,
    triangle: &[Vec<i32>],
    sub_solutions: &mut HashMap<(usize, usize), i32>,
) -> i32 {
    if let Some(s) = sub_solutions.get(&(row, col)) {
        return *s;
    }
    let node_val = triangle[row][col];
    if row == triangle.len() - 1 {
        return node_val;
    }
    let left_min = recursively_solve(row + 1, col, triangle, sub_solutions);
    let right_min = recursively_solve(row + 1, col + 1, triangle, sub_solutions);
    let sub_solution = node_val + left_min.min(right_min);
    sub_solutions.insert((row, col), sub_solution);
    sub_solution
}
