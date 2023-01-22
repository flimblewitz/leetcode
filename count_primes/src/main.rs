fn main() {
    println!("{}", Solution::count_primes(10));
    println!("{}", Solution::count_primes(11));
    println!("{}", Solution::count_primes(12));
    println!("{}", Solution::count_primes(0));
    println!("{}", Solution::count_primes(1));
    println!("{}", Solution::count_primes(499979));
    println!("{}", Solution::count_primes(5000000));
}

struct Solution;

impl Solution {
    pub fn count_primes(n: i32) -> i32 {
        // let's use the sieve of Eratosthenes
        // this is better because we don't have to repeat work across every positive integer less than n; we just do it all in one go
        // in contrast, the brute force approach encounters the same primes over and over as you iterate over all positive integers less than n
        let mut nums: Vec<(usize, bool)> = (2..n as usize).map(|num| (num, true)).collect();
        // println!("nums: {:?}", nums);
        let mut i = 0;
        while i < nums.len() {
            if let (num, true) = nums[i] {
                // println!("num: {}", num);
                let mut index_of_next_multiple = i + num;
                // println!("index_of_next_multiple: {}", index_of_next_multiple);
                while index_of_next_multiple < nums.len() {
                    // println!(" marking {} as not prime", multiple);
                    nums[index_of_next_multiple].1 = false;
                    index_of_next_multiple += num;
                    // println!(" index_of_next_multiple: {}", index_of_next_multiple);
                }
            }
            i += 1;
        }

        nums.iter().filter(|(_, is_prime)| *is_prime).count() as i32
    }
    // fn brute_force_count_primes(n: i32) -> i32 {
    //     (2..n)
    //         .filter(|num| *num == 2 || *num % 2 == 1)
    //         .filter(|num| Self::is_prime_using_o_sqrt_iteration(num))
    //         .count() as i32
    // }
    // fn is_prime_using_o_sqrt_iteration(num: &i32) -> bool {
    //     *num > 1
    //         && (*num == 2
    //             || !(2..=(*num as f32).sqrt() as i32)
    //                 .filter(|lesser_num| *lesser_num == 2 || lesser_num % 2 == 1)
    //                 .any(|lesser_num| *num % lesser_num == 0))
    // }
}
