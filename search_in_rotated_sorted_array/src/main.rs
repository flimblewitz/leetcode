fn main() {
    println!(
        "{}, should be 4",
        Solution::search(vec![4, 5, 6, 7, 0, 1, 2], 0)
    );
    println!(
        "{}, should be -1",
        Solution::search(vec![4, 5, 6, 7, 0, 1, 2], 3)
    );
    println!("{}, should be -1", Solution::search(vec![1], 0));
    println!(
        "{}, should be 0",
        Solution::search(vec![0, 1, 2, 3, 4, 5, 6, 7], 0)
    );
    println!("{}, should be 0", Solution::search(vec![3, 1], 3));
    println!("{}, should be 0", Solution::search(vec![3, 5, 1], 3));
    println!("{}, should be 0", Solution::search(vec![1], 1));
    println!("{}, should be 0", Solution::search(vec![1, 3], 1));
}

struct Solution {}
impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        // must run in O(log n)
        // I could unrotate the array and then do binary search, but that's just more work than necessary

        // I can tell immediately if an array is rotated by comparing the first and last items
        // if first > last, it's rotated
        // if the array is rotated, I know the min is less than the first item in the array

        // println!("");
        // println!("nums: {nums:?}, target: {target}");

        if nums.len() == 1 {
            return if nums[0] == target { 0 } else { -1 };
        }

        let first = nums[0];
        let last_index = nums.len() - 1;
        let last = nums[last_index];
        let mut index_of_min = 0;
        // if first > last, it's rotated
        if first > last {
            // since it's rotated, I have to binary search for the min
            let mut l: usize = 0;
            let mut r: usize = last_index;
            // as soon as we've narrowed down to a window of 1 item, we've found the min
            while l < r {
                let m = (l + r) / 2;
                // if the nums[m] is less than the first item in nums, then either nums[m] is the min or something between l and m is the min, so let's consider everything between l and m (inclusive) in the next iteration
                // otherwise, m can't possibly be the min, and everything between it and the first item must also be greater than the first item, meaning none of them are the min, so let's consider everything between m+1 and r (inclusive) in the next iteration
                if nums[m] < first {
                    r = m;
                } else {
                    l = m + 1;
                }
            }
            // println!("index of min: {l}");
            index_of_min = l;
        }

        // if the array's unrotated, the max is just the last item
        // else if the array is rotated, the max must be the item preceding the min (if there even is one; the array might only have a single item)
        let index_of_max = if index_of_min == 0 {
            nums.len() - 1
        } else {
            index_of_min.checked_sub(1).unwrap_or_default()
        };
        let max = nums[index_of_max];
        // and now we can consider only a strictly sorted window of array items
        // if the target is inclusively between the first and max items, let's just look in there. For unrotated arrays, that's the whole array. For rotated arrays, that's just the "front" part of the array
        // else, the array must be rotated, and we can just consider the "back" part of the array
        let (mut l, mut r) = match target >= first && target <= max {
            true => (0, index_of_max),
            false => (index_of_min, last_index),
        };
        // this is just binary search again, but this time it's a lot less painful because we're already considering a window that's strictly sorted
        while l < r {
            let m: usize = (l + r) / 2;
            if nums[m] < target {
                l = m + 1;
            } else {
                r = m;
            }
        }

        if target == nums[l] {
            return l as i32;
        }

        -1
    }
}
