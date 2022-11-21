use std::collections::HashMap;

fn main() {
    println!("{:?}", Solution::num_trees(1));
    println!("{:?}", Solution::num_trees(2));
    println!("{:?}", Solution::num_trees(3));
    println!("{:?}", Solution::num_trees(4));
    println!("{:?}", Solution::num_trees(18));
}

struct Solution {}
impl Solution {
    pub fn num_trees(n: i32) -> i32 {
        // so I don't need to return the bsts, just how many there are
        /*
        1: 1
        2: 2
        3: 5
        so it's the number of ways to make all subtrees given a root
        and that's the number of ways to make subtrees given each of its possible left values and each of its possible right values
        and what's more, the whole idea of bsts here is just a distraction. It's really just about how many ways to structure a hypothetical bst given a range of consecutive integers
        */
        let mut solutions: HashMap<i32, i32> = HashMap::new();
        recursively_get_num_trees(&mut solutions, n)
    }
}

fn recursively_get_num_trees(solutions: &mut HashMap<i32, i32>, n: i32) -> i32 {
    if n == 1 || n == 0 {
        return 1;
    }
    if let Some(&solution) = solutions.get(&n) {
        return solution;
    }
    let mut num = 0;
    for i in 1..=n {
        num += recursively_get_num_trees(solutions, i - 1)
            * recursively_get_num_trees(solutions, n - i);
    }
    solutions.insert(n, num);
    num
}
