fn main() {
    println!(
        "{:?}",
        Solution::num_islands(vec![
            vec!['1', '1', '1', '1', '0'],
            vec!['1', '1', '0', '1', '0'],
            vec!['1', '1', '0', '0', '0'],
            vec!['0', '0', '0', '0', '0']
        ])
    );
    println!(
        "{:?}",
        Solution::num_islands(vec![
            vec!['1', '1', '0', '0', '0'],
            vec!['1', '1', '0', '0', '0'],
            vec!['0', '0', '1', '0', '0'],
            vec!['0', '0', '0', '1', '1']
        ])
    );
}

struct Solution;

impl Solution {
    pub fn num_islands(grid: Vec<Vec<char>>) -> i32 {
        let mut observed_cells = vec![vec![false; grid[0].len()]; grid.len()];
        let mut num_islands = 0;
        for (row_index, row) in grid.iter().enumerate() {
            for (col_index, cell) in row.iter().enumerate() {
                if *cell == '1' && !observed_cells[row_index][col_index] {
                    num_islands += 1;
                    Self::traverse_island(row_index, col_index, &grid, &mut observed_cells);
                }
            }
        }
        num_islands
    }
    fn traverse_island(
        row_index: usize,
        col_index: usize,
        grid: &[Vec<char>],
        observed_cells: &mut [Vec<bool>],
    ) {
        let adjacent_cell_coords = [
            (row_index.checked_sub(1), Some(col_index)),
            (row_index.checked_add(1), Some(col_index)),
            (Some(row_index), col_index.checked_sub(1)),
            (Some(row_index), col_index.checked_add(1)),
        ];
        adjacent_cell_coords
            .into_iter()
            .filter_map(|coord| match coord {
                (Some(row_index), Some(col_index)) => Some((row_index, col_index)),
                _ => None,
            })
            .for_each(|(row_index, col_index)| {
                if row_index < grid.len()
                    && col_index < grid[0].len()
                    && grid[row_index][col_index] == '1'
                    && !observed_cells[row_index][col_index]
                {
                    observed_cells[row_index][col_index] = true;
                    Self::traverse_island(row_index, col_index, grid, observed_cells);
                }
            });
        //// confusingly, leetcode claims that this tuple is (&usize, &usize), so it needs the following for_each instead
        // .for_each(|(row_index, col_index)| {
        //     if *row_index < grid.len()
        //         && *col_index < grid[0].len()
        //         && grid[*row_index][*col_index] == '1'
        //         && !observed_cells[*row_index][*col_index]
        //     {
        //         observed_cells[*row_index][*col_index] = true;
        //         Self::traverse_island(*row_index, *col_index, grid, observed_cells);
        //     }
        // });
    }
}
