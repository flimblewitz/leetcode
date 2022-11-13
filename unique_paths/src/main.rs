use std::collections::HashMap;

fn main() {
    println!("{:?}", Solution::unique_paths(3, 7));
    println!("{:?}", Solution::unique_paths(3, 2));
    println!("{:?}", Solution::unique_paths(19, 13));
    println!("{:?}", Solution::unique_paths(51, 9));
}
struct Solution {}
impl Solution {
    pub fn unique_paths(m: i32, n: i32) -> i32 {
        // find all permutations of m downs and n rights
        // there are m-1 downs and n-1 rights

        // so I can recursively call a function with one of the numbers decremented

        // let's memoize to make this fast
        let mut sub_solutions: HashMap<(i32, i32), i32> = HashMap::new();

        recursively_get_num_paths(&mut sub_solutions, m - 1, n - 1, 0)
    }
}

fn recursively_get_num_paths(
    sub_solutions: &mut HashMap<(i32, i32), i32>,
    m: i32,
    n: i32,
    spaces: usize,
) -> i32 {
    if n == 0 || m == 0 {
        // println!("{}end", " ".repeat(spaces));
        return 1;
    }
    if let Some(&solution) = sub_solutions.get(&(m, n)) {
        return solution;
    }
    let num_paths_down = if m > 0 {
        // println!("{}down", " ".repeat(spaces));
        // recursively_get_num_paths(m - 1, n)
        recursively_get_num_paths(sub_solutions, m - 1, n, spaces + 1)
    } else {
        0
    };
    let num_paths_right = if n > 0 {
        // println!("{}right", " ".repeat(spaces));
        // recursively_get_num_paths(m, n - 1)
        recursively_get_num_paths(sub_solutions, m, n - 1, spaces + 1)
    } else {
        0
    };
    let num_paths = num_paths_down + num_paths_right;
    sub_solutions.insert((m, n), num_paths);
    num_paths
    // println!(
    //     "{}paths: {}",
    //     " ".repeat(spaces),
    //     num_paths
    // );
}
