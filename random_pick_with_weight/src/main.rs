fn main() {
    let s = Solution::new(vec![1]);
    assert_eq!(s.pick_index(), 0);
    println!("{}", s.pick_index());
    println!();

    let s = Solution::new(vec![1, 3]);
    println!("{}", s.pick_index());
    println!("{}", s.pick_index());
    println!("{}", s.pick_index());
    println!("{}", s.pick_index());
    println!("{}", s.pick_index());
    println!();

    let s = Solution::new(vec![3, 14, 1, 7]);
    println!("{}", s.pick_index());
    println!("{}", s.pick_index());
    println!("{}", s.pick_index());
    println!("{}", s.pick_index());
    println!("{}", s.pick_index());
    println!();
}

// I could make a huge array where values are duplicated as many times as there weight, and then randomly fetch from the huge array. But this is a brute force solution that will probably hit limits

// this problem is very sketchy because they can't reasonably test random answers
// I implemented this with an approach that passed test cases randomly, up to 55/57
// luckily, the current simpler approach worked

// this is an extremely rare problem where you need a non-standard crate
// annoyingly, leetcode only supports version 7.2 of this crate (I had to discover this by blindly running the code in their executor and seeing an error message that explained this), so that's what I'm using
use rand::Rng;

// this is an exceedingly rare problem where the struct actually needs to be included in the answer
struct Solution {
    total_weight: i32,
    subsequent_weight_sums: Vec<i32>,
}

// sorting is unnecessary for this
impl Solution {
    fn new(w: Vec<i32>) -> Self {
        let mut total_weight = 0;
        let mut subsequent_weight_sums = vec![];
        for weight in w {
            total_weight += weight;
            subsequent_weight_sums.push(total_weight);
        }
        Self {
            total_weight,
            subsequent_weight_sums,
        }
    }

    fn pick_index(&self) -> i32 {
        let mut rng = rand::thread_rng();
        let target = rng.gen_range(1, self.total_weight + 1);

        match self.subsequent_weight_sums.binary_search(&target) {
            Ok(i) => i as i32,
            Err(i) => i.min(self.subsequent_weight_sums.len() - 1) as i32,
        }
    }
}
