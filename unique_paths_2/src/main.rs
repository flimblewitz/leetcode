use std::collections::HashMap;

fn main() {
    println!(
        "{:?}",
        Solution::unique_paths_with_obstacles(vec![vec![0, 0, 0], vec![0, 1, 0], vec![0, 0, 0]])
    );
    println!(
        "{:?}",
        Solution::unique_paths_with_obstacles(vec![vec![0, 1], vec![0, 0]])
    );
    println!("{:?}", Solution::unique_paths_with_obstacles(vec![vec![1]]));
    println!(
        "{:?}",
        Solution::unique_paths_with_obstacles(vec![vec![1, 0]])
    );
    // test 30 of 41
    println!(
        "{:?}",
        Solution::unique_paths_with_obstacles(vec![
            vec![
                0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 1, 1, 0, 0, 1, 0, 1, 1, 0, 1, 0,
                0, 1, 0, 0, 0, 1, 0, 0
            ],
            vec![
                0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 1, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0
            ],
            vec![
                1, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 1,
                0, 1, 0, 0, 1, 0, 0, 0
            ],
            vec![
                1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 1, 0, 0, 0, 1, 0, 0, 0, 0,
                0, 0, 1, 0, 0, 1, 0, 1
            ],
            vec![
                0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 1, 0, 0
            ],
            vec![
                0, 0, 1, 0, 0, 0, 1, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1, 1,
                0, 0, 1, 0, 0, 1, 0, 0
            ],
            vec![
                0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 1, 0, 1, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 1, 0, 0, 1, 0, 0
            ],
            vec![
                1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0,
                1, 0, 1, 1, 0, 0, 0, 1
            ],
            vec![
                0, 0, 0, 0, 1, 0, 0, 1, 0, 1, 1, 1, 0, 0, 0, 1, 0, 0, 1, 0, 1, 0, 0, 0, 0, 1, 1, 1,
                0, 0, 0, 0, 0, 0, 0, 0
            ],
            vec![
                0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 1, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 1, 1, 0, 0, 0
            ],
            vec![
                1, 0, 1, 0, 1, 1, 0, 1, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 1, 1, 0, 0,
                0, 0, 1, 0, 0, 0, 1, 0
            ],
            vec![
                0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 1, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 1, 1, 0, 0, 0, 0, 0,
                1, 0, 0, 1, 0, 0, 0, 1
            ],
            vec![
                0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0,
                0, 1, 0, 0, 1, 0, 0, 0
            ],
            vec![
                1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0
            ],
            vec![
                0, 0, 1, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 1, 0
            ],
            vec![
                0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 1, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0,
                0, 1, 0, 0, 0, 0, 0, 0
            ],
            vec![
                0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 1, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0,
                1, 0, 0, 0, 0, 0, 0, 0
            ],
            vec![
                0, 0, 0, 0, 0, 1, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 1, 0
            ],
            vec![
                0, 0, 1, 0, 1, 0, 1, 0, 0, 0, 0, 0, 1, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0
            ],
            vec![
                0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 1, 0, 0, 0, 1, 0, 0, 0,
                1, 0, 0, 0, 1, 0, 0, 0
            ],
            vec![
                0, 0, 0, 1, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 1, 1, 1, 0, 1, 1, 1, 0, 0, 1, 0, 1, 0, 1,
                1, 1, 0, 0, 0, 0, 0, 0
            ],
            vec![
                0, 0, 1, 0, 0, 0, 0, 1, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0
            ],
            vec![
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 1, 0, 0, 0, 0, 0
            ],
            vec![
                0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0
            ],
            vec![
                1, 1, 0, 0, 0, 0, 1, 0, 0, 1, 1, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 1, 0
            ],
            vec![
                0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0,
                0, 0, 1, 0, 0, 1, 0, 0
            ],
            vec![
                0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 1, 1, 1, 0, 0, 0, 0, 0, 0,
                1, 0, 0, 0, 1, 1, 0, 0
            ],
            vec![
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1, 0,
                0, 0, 0, 0, 0, 0, 0, 0
            ]
        ])
    );
}
struct Solution {}
impl Solution {
    pub fn unique_paths_with_obstacles(obstacle_grid: Vec<Vec<i32>>) -> i32 {
        // so I guess I have to recursively do this. The obstacles make memoization hard, so I'll skip that for pass 1
        // I can still actually memoize by coordinates; the obstacles are consistent, so it doesn't matter
        if *obstacle_grid.last().unwrap().last().unwrap() == 1
            || *obstacle_grid.first().unwrap().first().unwrap() == 1
        {
            return 0;
        }
        let mut sub_solutions: HashMap<(usize, usize), i32> = HashMap::new();
        recursively_get_num_paths(&mut sub_solutions, &obstacle_grid, 0, 0)
    }
}

fn recursively_get_num_paths(
    sub_solutions: &mut HashMap<(usize, usize), i32>,
    obstacle_grid: &Vec<Vec<i32>>,
    r: usize,
    c: usize,
) -> i32 {
    if let Some(solution) = sub_solutions.get(&(r, c)) {
        return *solution;
    }
    if r == obstacle_grid.len() - 1 && c == obstacle_grid[0].len() - 1 {
        return 1;
    }
    let num_paths_right = match obstacle_grid
        .get(r)
        .and_then(|row| row.get(c + 1).map(|val| *val != 1))
    {
        Some(true) => recursively_get_num_paths(sub_solutions, obstacle_grid, r, c + 1),
        _ => 0,
    };
    let num_paths_down = match obstacle_grid
        .get(r + 1)
        .and_then(|row| row.get(c).map(|val| *val != 1))
    {
        Some(true) => recursively_get_num_paths(sub_solutions, obstacle_grid, r + 1, c),
        _ => 0,
    };
    let num_paths = num_paths_right + num_paths_down;
    sub_solutions.insert((r, c), num_paths);
    num_paths
}
