fn main() {
    assert_eq!(Solution::length_of_lis(vec![10, 9, 2, 5, 3, 7, 101, 18]), 4);
    assert_eq!(Solution::length_of_lis(vec![0, 1, 0, 3, 2, 3]), 4);
    assert_eq!(Solution::length_of_lis(vec![7, 7, 7, 7, 7, 7, 7]), 1);
    assert_eq!(Solution::length_of_lis(vec![10, 9, 2, 5, 3, 4]), 3);
    assert_eq!(Solution::length_of_lis(vec![10, 9, 2, 5, 3, 4, 1]), 3);
    assert_eq!(
        Solution::length_of_lis(vec![3, 5, 6, 2, 5, 4, 19, 5, 6, 7, 12]),
        6
    );
}
struct Solution;

// in my original attempts, I tried to enumerate nums and then sort it by value. But that was a dead end. I couldn't find a way to reliably identify the lis from that without failing to maintain the O(n log(n)) time constraint

// this implementation is sadly based on an existing solution
impl Solution {
    pub fn length_of_lis(nums: Vec<i32>) -> i32 {
        // println!();
        // println!("{:?}", nums);

        // I'll maintain the longest observed increasing subsequence as I iterate through nums
        // so lis is [nums[i], nums[j], num[k], ...], where i < j < k
        // for each num, I'll look for the index where it would be sorted into lis. Let's call this index n
        // if n is out of bounds, then clearly I can safely push it to the end of lis
        // if n is in bounds, that means there is some nums[j] such that n > j and nums[n] < nums[j]
        // if I replace nums[j] in [nums[i], nums[j], num[k], ...] with nums[n], then I've broken the invariant, because n > k
        // if j is the last index, though, then I can safely replace nums[j] with nums[n], and in fact I WANT to do that because it will open up the possibility for more items to be in the subsequence later that are greater than nums[n] but not nums[j]
        // if we already have the observed num in lis, we can immediately disregard it. This duplicated occurrence cannot possibly exist in a longer increasing subsequence. So we really only care about wholly new nums

        // for each num, I'll look for the index where it would be sorted into lis
        // if we already have the observed num in lis, we can immediately disregard it. This duplicated occurrence cannot possibly exist in a longer increasing subsequence. So we really only care about wholly new nums
        // if it belongs out of bounds, that's great. It means that we can push it onto the end of lis
        // if it belongs IN bounds at index i, though, that means our current lis of is failing to grow. If we had to care about the actual contents of lis, we could maintain distinct potential lis incarnations where each was [nums[a], nums[b], num[c], ...], where a < b < c. However, because we don't actually care about the contents of lis, we can drastically reduce the space requirement by imagining all potential lis incarnations as being superimposed. In other words, we can use a singular array and just keep overwriting its elements with abandon. If we replace nums[a] and nums[b] with nums[i] and nums[j], and then run out of nums to consider, it doesn't matter, because whatever potential lis incarnation got us all the way to nums[c] earlier has still been "tracked" by the presence of that third item in the superimposed lis array. And even if we replace nums[c] with a nums[k] as well, if there's ever some nums[l] that can follow the new nums[k], it doesn't matter that we've "lost" nums[c], because we're clearly doing better now. And if some nums[d] and nums[e] show up, if they could follow nums[c], then they must be able to follow nums[k] too
        // again, to make this easier to reason about, we could track each potential lis incarnation separately. But that would slow things down to O(n^2 log(n)) and would use O(n^2) space, so let's not do it
        let mut superimposed_lis: Vec<i32> = vec![];
        for num in nums {
            // println!("superimposed_lis: {:?}", superimposed_lis);
            // println!("num: {}", num);
            if let Err(i) = superimposed_lis.binary_search(&num) {
                if i == superimposed_lis.len() {
                    superimposed_lis.push(num);
                } else {
                    superimposed_lis[i] = num;
                }
            }
        }

        superimposed_lis.len() as i32
    }
}
