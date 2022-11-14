fn main() {
    // let mut v = vec![2, 0, 2, 1, 1, 0];
    let mut v = vec![1, 2, 2, 1, 2, 0];
    Solution::sort_colors(&mut v);
    println!("{:?}", v);
}
struct Solution {}
impl Solution {
    pub fn sort_colors(nums: &mut Vec<i32>) {
        recursively_sort_slice(nums);
    }
}

// mergesort
fn recursively_sort_slice(nums: &mut [i32]) {
    // if you have nothing to sort, great
    if nums.is_empty() || nums.len() == 1 {
        return;
    }
    // sort both halves individually
    let mid = nums.len() / 2;
    recursively_sort_slice(&mut nums[..mid]);
    recursively_sort_slice(&mut nums[mid..]);
    // we will now choose one half to iterate over and another to "dissolve" as necessary. If any items in the iterated array are undesirably positioned compared to an item in the dissolved array, pull that dissolved array value out and cram it into the iterated array. Then keep iterating over the iterated array.
    // you can use either the left half or the right half for either role. It doesn't matter. I assume that most people iterate over the left array
    // so l=0, r=mid
    // if the lth item <= the rth item, increment l
    // else, save the rth item, shift every item from indexes [l..r) right by one, then set the lth item to the original rth item. Increment r to maintain the loop invariant that the right array, our "dissolved" array, is always sorted. Meanwhile, the left half array is guaranteed to be sorted as well because of the shift operation. Since that original rth item (now the lth item) is guaranteed to be <= the r+1th item, there's no need to compare it to the new lth item, so we can just increment l as well (but we technically don't have to because the next loop iteration will do the same).
    // we're done when l and r converge or r == nums.len() because the right half array has totally dissolved into the left one
    let mut l = 0;
    let mut r = mid;
    while l < r && r < nums.len() {
        match nums[l].cmp(&nums[r]) {
            std::cmp::Ordering::Less | std::cmp::Ordering::Equal => l += 1,
            std::cmp::Ordering::Greater => {
                let tmp = nums[r];
                for i in (l + 1..=r).rev() {
                    nums[i] = nums[i - 1];
                }
                nums[l] = tmp;
                r += 1;
                l += 1;
            }
        }
    }
}
