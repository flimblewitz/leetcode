fn main() {
    assert_eq!(Solution::triangle_number(vec![2, 2, 3, 4]), 3);
    assert_eq!(Solution::triangle_number(vec![1]), 0);
}
struct Solution;

impl Solution {
    pub fn triangle_number(mut nums: Vec<i32>) -> i32 {
        // what makes three numbers possible sides to the same triangle?
        // the summed length of two sides must exceed the length of the last one
        // x + y > z
        // if the array is sorted, things get easier
        // so given some x and some y after x, we consider any z after y where x + y > z
        // the dumb way to solve this is to iterate for all valid zs using _brute_force_get_number_of_valid_zs
        // the smart way is to binary search for x+y, see if the preceding index is in bounds and is less than x+y, and just use z_index-y_index to get the count of valid zs given an x and a y

        if nums.len() < 3 {
            return 0;
        }

        nums.sort_unstable();

        let mut triangle_number_count = 0;

        for x_index in 0..nums.len() - 2 {
            for y_index in x_index + 1..nums.len() - 1 {
                triangle_number_count +=
                    Self::binary_search_get_number_of_valid_zs(&nums, x_index, y_index);
            }
        }

        triangle_number_count
    }

    fn binary_search_get_number_of_valid_zs(nums: &[i32], x_index: usize, y_index: usize) -> i32 {
        let first_val_after_possible_zs = nums[x_index] + nums[y_index];
        let first_index_after_z_indexes = match nums.binary_search(&first_val_after_possible_zs) {
            Ok(mut i) => {
                // the built in binary search isn't guaranteed to give us the first occurrence of the target if there are multiple matches, so let's just skip over the duplicates lazily. We're only bothering to do this because this problem doesn't want us to find unique triplets
                while i > y_index + 1 && nums[i - 1] == first_val_after_possible_zs {
                    i -= 1;
                }
                i
            }
            Err(i) => i,
        };
        if first_index_after_z_indexes > y_index + 1 {
            return (first_index_after_z_indexes - y_index - 1) as i32;
        }
        0
    }

    fn _brute_force_get_number_of_valid_zs(nums: &[i32], x_index: usize, y_index: usize) -> i32 {
        let mut count = 0;
        let mut z_index = y_index + 1;
        while z_index < nums.len() && nums[z_index] < nums[x_index] + nums[y_index] {
            count += 1;
            z_index += 1;
        }
        count
    }
}
