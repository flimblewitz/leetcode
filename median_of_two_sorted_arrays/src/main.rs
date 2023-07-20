fn main() {
    assert_eq!(
        Solution::find_median_sorted_arrays(vec![1, 2, 3, 4, 6, 7], vec![5]),
        4f64
    );
    assert_eq!(
        Solution::find_median_sorted_arrays(vec![1, 2, 3, 6, 7], vec![5, 5]),
        5f64
    );
    assert_eq!(
        Solution::find_median_sorted_arrays(vec![1, 2, 3, 4, 5, 6, 7], vec![8, 9, 10]),
        5.5f64
    );
    assert_eq!(
        Solution::find_median_sorted_arrays(vec![1, 2, 3, 4, 5, 6, 7], vec![1, 9, 10]),
        4.5f64
    );
    assert_eq!(
        Solution::find_median_sorted_arrays(vec![1, 3], vec![2]),
        2f64
    );
    assert_eq!(
        Solution::find_median_sorted_arrays(vec![1, 2], vec![3, 4]),
        2.5f64
    );
    assert_eq!(
        Solution::find_median_sorted_arrays(vec![1, 2, 3, 4], vec![0, 6]),
        2.5f64
    );
    assert_eq!(
        Solution::find_median_sorted_arrays(vec![0, 2, 3, 5], vec![2, 4]),
        2.5f64
    );
    assert_eq!(
        Solution::find_median_sorted_arrays(vec![1, 4], vec![0, 6]),
        2.5f64
    );
    assert_eq!(
        Solution::find_median_sorted_arrays(vec![1, 2], vec![-1, 3]),
        1.5f64
    );
    assert_eq!(
        Solution::find_median_sorted_arrays(vec![1], vec![2, 3, 4, 5]),
        3.0
    );
}
struct Solution;

// note that my solution was based on the leetcode editorial for this problem. I got pretty close on my own but just didn't understand how to close it out for the last half or so of the test cases, and identifying the median at the end had me pretty stumped. Ironically, while it showed me the general shape for how to solve the problem, I still ended up having to puzzle out several aspects of the solution myself because the editorial made several significant mistakes and was not particularly well written otherwise
impl Solution {
    pub fn find_median_sorted_arrays(mut a: Vec<i32>, mut b: Vec<i32>) -> f64 {
        // println!();
        // println!("a: {a:?}");
        // println!("b: {b:?}");

        // let's refer to the length of a as m
        // let's refer to the length of b as n

        // for some reason, the editorial insists that the the array has (m+n+1)/2 elements. Where did the plus one come from??? That's just not true!

        // we are going to use indexes a_partition and b_partition to divide a and b into a portion of smaller values anda portion of larger values each, where the smaller portion strictly precedes the partition index
        // let's call the smaller portion of a "a_left" and the larger portion "a_right", and likewise for b
        // note that the length of a_left is the same as the value of a_partition, and likewise for b
        // let's call the last element of a_left (at index a_partition - 1) "a_left_max" and the first element of a_right (at index a_partition) "a_right_min", and likewise for b
        // let's refer to a_left_max, a_right_min, b_left_max, and b_right_min as the "partition edge values"
        // note that theoretically a_partition or b_partition could end up at the first or last index of their array. Things get much easier if we pretend that that any out-of-bounds values are i32::MIN on the left and i32::MAX on the right. This is safe because that makes them "absolute" values that we'll never try to push past, and when we eventually derive the median from the partition edge values, it will be the innermost partition edge value(s), so it will never end up being misidentified as one of these dummy values

        // as long as we ensure that the combination of a_left and b_left has (m+n)/2 items in it, a's and b's partition edge values are all potential medians
        // so we only need to - and want to - consider cases where len(a_left) + len(b_left) = (m+n)/2
        // an easy way to maintain that is by only bothering to vary a_partition directly and setting b_partition = (m+n)/2 - a_partition
        // that way we have as many as a_partition elements in a_left and (m+n)/2 - a_partition in b_left
        // and therefore len(a_left) + len(b_left) == (m+n)/2
        // this is safe _at first_ because a_partition == m/2, meaning at first b_partition == (m+n)/2 - m/2 => n/2
        // we'll have to ensure we don't ever assign a value to a_partition that could break b_partition by making it <0 or >=n
        // we can accomplish that safety guarantee by ensuring that m <= n
        // let's do that now
        if a.len() > b.len() {
            std::mem::swap(&mut a, &mut b);
        }
        // at greatest, a_partition would be m-1 <= n-1. That would mean b_partition == (2n-1)/2 - (n-1) => 1/2
        // at least, a_partition would be 0. That would mean b_partition == n/2

        // the challenge is to find partition edge values such that the partition edge values are straddling each other, because that means that the combined median can be derived from them and no other values in the combined array
        // to express that in math: a_left_max <= b_right_min && b_left_max <= a_right_min
        // recall that we're already guaranteeing len(a_left) + len(b_left) == (m+n)/2

        // once that's done, we can discern the median by considering the lengths of a, the length of b, and the partition edge values themselves

        // so for each iteration, we'll just focus on finding the perfect a_partition knowing that the only corresponding b_partition that we care about can be derived from it
        // we'll know what to do for each iteration by considering the partition edge values. There are 3 cases

        // 1. if we're straddling, bingo. We can calculate the median and return it
        // otherwise, we're not straddling, and we need to refine our search

        // 2. if a_left_max > b_right_min, then a_left_max and all of a_right can be dismissed as potential members of the smaller half of the combined array, because thanks to them we're failing to straddle our partition edge values
        // in other words, a_left_max has failed us as a partition edge value, and we can only possibly do better if we look for a smaller one. We can't go larger because any increase to a_partition will subsequently decrease b_partition, and as such a_left_max > b_right_min would still be true
        // so we can set r = a_partition - 1 and move onto the next iteration

        // 3. otherwise, b_left_max > a_right_min
        // this means that a_right_min and all of a_left must belong in the smaller half of the combined array, because thanks to them we're failing to straddle our partition edge values
        // in other words, a_right_min has failed us as a partition edge value, and we can only possibly do better if we look for a larger one. We can't go smaller because any decrease to a_partition will subsequently increase b_partition, and as such b_left_max > a_right min would still be true
        // so we can set l = a_partition + 1 and move onto the next iteration

        let mut l = 0;
        let mut r = a.len();
        while l <= r {
            let a_partition = (l + r) / 2;
            let b_partition = (a.len() + b.len()) / 2 - a_partition;

            // println!("a_partition: {}", a_partition);
            let a_left_max = Self::get_preceding_item(&a, a_partition);
            let a_right_min = Self::get_array_item(&a, a_partition);

            // println!("b_partition: {}", b_partition);
            let b_left_max = Self::get_preceding_item(&b, b_partition);
            let b_right_min = Self::get_array_item(&b, b_partition);

            if a_left_max <= b_right_min && b_left_max <= a_right_min {
                // we're straddling! That means we can finally identify our median!
                if (a.len() + b.len()) % 2 == 0 {
                    // if we have an even-length combined array, then the median is the average of the closest straddling values
                    return (a_left_max.max(b_left_max) + a_right_min.min(b_right_min)) as f64
                        / 2.0;
                } else {
                    // otherwise, if we have an odd-length combined array, then we still have 4 partition edge values to consider
                    // note that one of a or b has an odd length. For the odd-length array, only its _right_min value is a potential median; we only considered its _left_max because we needed to use the "straddling" logic to find our potential medians
                    // this means that the 4 straddling partition edge values are "offset" by one to the left, e.g. [ _, w, x, y, z, _, _]
                    // this means that the median must be the lesser of the two rightmost partition edge values, which are a_right_min and b_right_min
                    // note that the leetcode editorial actually got this wrong by taking the max of a_left_max and b_left_max (represented by the following commented-out return statement)
                    // return a_left_max.max(b_left_max) as f64;
                    // println!("a_partition: {}", a_partition);
                    // println!("a_right_min: {}", a_right_min);
                    // println!("b_partition: {}", b_partition);
                    // println!("b_right_min: {}", b_right_min);
                    return a_right_min.min(b_right_min) as f64;
                }
            } else if a_left_max > b_right_min {
                r = a_partition - 1;
            } else {
                l = a_partition + 1;
            }
        }

        panic!("If the two arrays truly are sorted, then the algorithm shouldn't have gotten this far. It must be flawed. Oops.")
    }

    fn get_array_item(arr: &[i32], i: usize) -> i32 {
        if i < arr.len() {
            arr[i]
        } else {
            i32::MAX
        }
    }

    fn get_preceding_item(arr: &[i32], i: usize) -> i32 {
        if let Some(i_minus_one) = i.checked_sub(1) {
            arr[i_minus_one]
        } else {
            i32::MIN
        }
    }
}
