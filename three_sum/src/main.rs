fn main() {
    println!("{:?}", Solution::three_sum(vec![-1, 0, 1, 2, -1, -4]));
    println!(
        "{:?}",
        Solution::three_sum(vec![-4, -2, 1, -5, -4, -4, 4, -2, 0, 4, 0, -2, 3, 1, -5, 0])
    );
    println!("{:?}", Solution::three_sum(vec![1, 1, -2]));
}

struct Solution {}
impl Solution {
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut nums = nums;
        nums.sort();
        let mut res = vec![];

        for i in 0..nums.len() {
            if i > 0 && nums[i] == nums[i - 1] {
                continue;
            }
            let mut j = i + 1;
            let mut k = nums.len() - 1;

            while j < k {
                // after the first values of j and k, let's avoid all repeated values of num[j] and num[k]
                if j > i + 1 && nums[j] == nums[j - 1] {
                    j += 1;
                    continue;
                }
                if k < nums.len() - 1 && nums[k] == nums[k + 1] {
                    k -= 1;
                    continue;
                }

                let sum = nums[i] + nums[j] + nums[k];
                if sum == 0 {
                    // if there's a match, congratulations. But there might still be possible values of j and k that result in a sum of 0, so tighten the inner window and try again
                    res.push([nums[i], nums[j], nums[k]].to_vec());
                    j += 1;
                    k -= 1;
                } else if sum < 0 {
                    j += 1;
                } else {
                    k -= 1;
                }
            }
        }

        res
    }
}
