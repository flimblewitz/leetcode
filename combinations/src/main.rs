fn main() {
    // println!("{:?}", Solution::combine(1, 1));
    // println!("{:?}", Solution::combine(1, 3)); // should be none
    // println!("{:?}", Solution::combine(3, 1));
    println!("{:?}", Solution::combine(4, 2));
}
struct Solution {}
impl Solution {
    pub fn combine(n: i32, k: i32) -> Vec<Vec<i32>> {
        recursively_solve(1, n, k)
    }
}

fn recursively_solve(start: i32, n: i32, k: i32) -> Vec<Vec<i32>> {
    if k == 1 {
        return (start..=n).map(|num| vec![num]).collect();
    }
    let mut v = vec![];

    // if I need all k combinations for the given set, I should first consider all possible combinations of k-1 elements without the given number. Then I should combine the given number with all of them.
    // then I should iterate the given number to find all k element combinations that don't include that previous given number
    for num in start..=n {
        let mut sub_combinations = recursively_solve(num + 1, n, k - 1);
        sub_combinations
            .iter_mut()
            .for_each(|sub_combination| sub_combination.insert(0, num));
        v.append(&mut sub_combinations);
    }

    v
}
