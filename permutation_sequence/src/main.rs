fn main() {
    println!("{:?}", Solution::get_permutation(3, 1));
    println!("{:?}", Solution::get_permutation(3, 2));
    println!("{:?}", Solution::get_permutation(3, 3));
    println!("{:?}", Solution::get_permutation(3, 4));
    println!("{:?}", Solution::get_permutation(3, 5));
    println!("{:?}", Solution::get_permutation(3, 6));
    println!("{:?}", Solution::get_permutation(4, 9));
}

struct Solution {}
impl Solution {
    pub fn get_permutation(n: i32, k: i32) -> String {
        // assume k starts at 0 and the 0th permutation is just the first one
        // given n items, there are n different (n-1)! permutations, each for a different leading item
        // I want to skip ahead to just the subset with the kth permutation
        // so I want to know what (n-1)! is and then swap index 0 with index 1 + k/((n-1)!)
        // then I take that subarray, sort it, and recursively ask for the k % (n-1)! th permutation of that subarray
        // eventually, the subarray will be of length 1, at which point I'm done
        let mut permuation: Vec<u32> = (1..=(n as u32)).collect();
        recursively_form_kth_permutation(&mut permuation[..], k as usize - 1);
        permuation
            .iter()
            .map(|&int| char::from_digit(int, 10).unwrap())
            .collect()
    }
}

fn recursively_form_kth_permutation(permutation: &mut [u32], k: usize) {
    let len = permutation.len();
    if len <= 1 {
        return;
    }
    let f = factorial(len - 1);
    let perm_class = k / f;
    permutation.swap(0, perm_class);
    let permutation = &mut permutation[1..];
    permutation.sort();
    recursively_form_kth_permutation(permutation, k % f);
}

fn factorial(num: usize) -> usize {
    match num {
        0 => 1,
        1 => 1,
        _ => factorial(num - 1) * num,
    }
}
