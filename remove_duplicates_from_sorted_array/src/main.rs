fn main() {
    let mut v = vec![1, 1, 2];
    println!("{}, {:?}", Solution::remove_duplicates(&mut v), v);
    let mut v = vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4];
    println!("{}, {:?}", Solution::remove_duplicates(&mut v), v);
    let mut v = vec![1, 1];
    println!("{}, {:?}", Solution::remove_duplicates(&mut v), v);
}

struct Solution {}
impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let mut i = 0;
        loop {
            // let's not go out of bounds
            if i > nums.len() - 1 {
                break;
            }
            // otherwise, if it's the second index or greater, compare to the previous item
            if i > 0 && nums[i - 1] == nums[i] {
                nums.remove(i);
            } else {
                // if it's the first item or the previous item is different, consider the next index in the next iteration
                i += 1;
            }
        }
        nums.len() as i32
    }
}
