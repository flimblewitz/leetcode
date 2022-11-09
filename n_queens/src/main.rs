fn main() {
    // println!("{:?}", Solution::solve_n_queens(2));
    println!("{:?}", Solution::solve_n_queens(4));
}

struct Solution {}
impl Solution {
    pub fn solve_n_queens(n: i32) -> Vec<Vec<String>> {
        let n = n as usize;
        let solutions = recursive_solve(0, &vec![false; n], &vec![false; n], &vec![]);

        println!("{:?}", solutions);

        solutions
            .iter()
            .map(|coords| {
                coords
                    .iter()
                    .map(|coord| {
                        let mut v: Vec<String> = vec![".".into(); n as usize];
                        v[coord.1] = "Q".into();
                        v.join("")
                    })
                    .collect()
            })
            .collect()
    }
}

// this only beats 5.19% of solutions for runtime
fn recursive_solve(
    starting_row_index: usize,
    used_rows: &Vec<bool>,
    // starting_column_index: usize,
    used_columns: &Vec<bool>,
    starting_combination: &Vec<(usize, usize)>,
) -> Vec<Vec<(usize, usize)>> {
    let mut all_valid_combinations: Vec<Vec<(usize, usize)>> = vec![];

    for (r, _) in used_rows
        .iter()
        .enumerate()
        .filter(|(index, &used)| *index >= starting_row_index && !used)
    {
        let mut cloned_used_rows = used_rows.clone();
        cloned_used_rows[r] = true;
        for (c, _) in used_columns.iter().enumerate().filter(|(_, &used)| !used) {
            // if diagonals, abort
            if starting_combination
                .iter()
                .any(|(x, y)| r.max(*x) - r.min(*x) == c.max(*y) - c.min(*y))
            {
                continue;
            }

            let mut cloned_used_columns = used_columns.clone();
            cloned_used_columns[c] = true;

            let mut cloned_starting_combination = starting_combination.clone();
            cloned_starting_combination.push((r, c));

            let valid_combinations = if cloned_used_rows.iter().all(|&used| used) {
                vec![cloned_starting_combination.clone()]
            } else {
                recursive_solve(
                    r + 1,
                    &cloned_used_rows,
                    &cloned_used_columns,
                    &cloned_starting_combination,
                )
            };

            all_valid_combinations = vec![all_valid_combinations, valid_combinations].concat();
        }
    }
    all_valid_combinations
}
