fn main() {
    println!("{}", Solution::max_area(vec![1, 8, 6, 2, 5, 4, 8, 3, 7]));
}

struct Solution {}
impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut max_area = 0;
        for l in 0..(height.len() - 1) {
            for r in ((l + 1)..height.len()).rev() {
                if max_area > height[l] * (r - l) as i32 {
                    // we cannot exceed the current max_area no matter how much we walk r to the left
                    break;
                }
                max_area = max_area.max(height[l].min(height[r]) * (r - l) as i32);
            }
        }
        max_area
    }
}
