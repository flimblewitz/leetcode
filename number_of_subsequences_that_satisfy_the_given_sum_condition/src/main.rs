fn main() {
    assert_eq!(Solution::num_subseq(vec![3, 5, 6, 7], 9), 4);
    assert_eq!(Solution::num_subseq(vec![3, 3, 6, 8], 10), 6);
    assert_eq!(Solution::num_subseq(vec![2, 3, 3, 4, 6, 7], 12), 61);
    assert_eq!(Solution::num_subseq(vec![7, 10, 7, 3, 7, 5, 4], 12), 56);
    assert_eq!(
        Solution::num_subseq(vec![12, 15, 10, 4, 5, 7, 6, 6, 9, 13, 1, 8, 2, 5, 2], 24),
        32757
    );
    assert_eq!(
        Solution::num_subseq(
            vec![
                14, 4, 6, 6, 20, 8, 5, 6, 8, 12, 6, 10, 14, 9, 17, 16, 9, 7, 14, 11, 14, 15, 13,
                11, 10, 18, 13, 17, 17, 14, 17, 7, 9, 5, 10, 13, 8, 5, 18, 20, 7, 5, 5, 15, 19, 14
            ],
            22
        ),
        272187084
    );
    assert_eq!(
        Solution::num_subseq(
            vec![
                82, 30, 56, 28, 63, 24, 17, 50, 45, 95, 50, 41, 10, 59, 59, 17, 39, 11, 36, 64, 44,
                16, 56, 31, 83, 100, 12, 69, 13, 46, 58, 92, 55, 71, 33, 99, 78, 13, 17, 50, 82,
                33, 12, 22, 48, 48, 77, 71, 11, 83, 26, 51, 26, 56, 51, 82, 54, 50, 13, 64, 83, 48,
                31, 28, 33, 89, 60, 22, 25, 35, 89, 80, 65, 92, 52, 29, 64, 96, 98, 76, 93, 77, 90,
                72, 49, 62, 78, 41, 99, 22, 36, 64, 23, 76, 71, 69, 83, 84, 77, 72, 24, 21, 89, 37,
                20, 59, 48, 78, 68, 60, 25, 93, 38, 83, 16, 85, 10, 20, 34, 98, 73, 16, 45, 99, 75,
                29, 12, 27, 35, 54, 72, 10, 99, 61, 92, 67, 43, 31, 27, 88, 51, 92, 75, 59, 47, 48,
                48, 22, 61, 69, 69, 90, 61, 85, 48, 83, 79, 96, 27, 48, 24, 20, 35, 52, 49, 58, 84,
                71, 30, 89, 72, 74, 72, 87, 31, 43, 66, 18, 92, 63, 48, 71, 75, 42, 37, 53, 76, 79,
                87
            ],
            125
        ),
        945569954
    );
}
struct Solution;

// my first solution for this used combinatorics to count all the possible ways to form subsequences given valid first and last items in a subsequence
// unfortunately, that was doomed to hit overflows during multiplication
// I tried using u128 as a type, but that only postponed the problem to test cases with bigger numbers
// this is a good lesson about combinatorics being a potential dead end
// the alternative to combinatorics was just to realize that there were 2^n choices for including/excluding n items in a subsequence
// another key aspect of the solution is avoiding overflow for 2^n itself by using the allowed modulo division on each power of 2 in advance. This is lame, but there just isn't a good way to know what powers of 2 are too big until it's too late. Precomputing the whole set of powers of 2 that you could possibly use - with the modulo division applied - just solves that problem in a surprisingly reasonable amount of time and space
impl Solution {
    pub fn num_subseq(mut nums: Vec<i32>, target: i32) -> i32 {
        // return the number of non-empty subseqs such that min + max <= target

        // this is only practical to do if nums is sorted
        nums.sort_unstable();

        // the way this plays out, we end up retrieving powers of 2
        // annoyingly, these powers of 2 get big enough that we can't even fit them in our primitive values
        // we have been given permission to modulo divide the answer by 10^7 + 9, so let's just do that up front since there isn't really a good way to handle such cases on-demand
        let mut powers_of_2: Vec<i32> = vec![1];
        (1..nums.len())
            .for_each(|i| powers_of_2.push((powers_of_2[i - 1] * 2) % (10i32.pow(9) + 7)));

        // println!();
        // println!("nums: {:?}, target: {}", nums, target);
        let mut count: i32 = 0;
        for i in 0..nums.len() {
            // println!("count: {}", count);
            // println!("i: {}", i);

            // we can use a slight optimization
            if 2 * nums[i] > target {
                // println!("giving up; 2 * {} > target", nums[i]);
                break;
            }

            // j will be the first index after i for which nums[i] + nums[j] > target
            // we can get it by binary searching the slice after i
            // but then, we need to add i+1 to the resulting number because that slice purposefully excluded those first i+1 items
            let j = nums[(i + 1)..]
                .binary_search_by(|num| match (nums[i] + *num).cmp(&target) {
                    std::cmp::Ordering::Less | std::cmp::Ordering::Equal => {
                        std::cmp::Ordering::Less
                    }
                    std::cmp::Ordering::Greater => std::cmp::Ordering::Greater,
                })
                .unwrap_err()
                + i
                + 1;
            // println!("j: {}", j);

            // so given i and j, all possible subseqs consist of i followed by any number of elements up to j, excluding j
            // so there are j-i-1 such elements between and excluding i and j
            // for each, we can choose to omit or include it in a subseq
            // so that's 2^(j-i-1) possible subseqs given i and j
            count += powers_of_2[j - i - 1];
            // let's just use the modulo now just in case
            count = count % (10i32.pow(9) + 7);
        }

        (count % (10i32.pow(9) + 7)) as i32
    }
}
