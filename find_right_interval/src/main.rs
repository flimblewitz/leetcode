fn main() {
    assert_eq!(Solution::find_right_interval(vec![vec![1, 2]]), vec![-1]);
    assert_eq!(
        Solution::find_right_interval(vec![vec![3, 4], vec![2, 3], vec![1, 2]]),
        vec![-1, 0, 1]
    );
    assert_eq!(
        Solution::find_right_interval(vec![vec![1, 4], vec![2, 3], vec![3, 4]]),
        vec![-1, 2, -1]
    );
}

struct Solution;
impl Solution {
    pub fn find_right_interval(intervals: Vec<Vec<i32>>) -> Vec<i32> {
        // so the first thing I should do is sort the intervals
        // I'll sort them by their starts, because I'm searching for an "ideal" corresponding start for each of them

        // but the whole point is to find the index of the other interval with that "ideal" start, so when I do the sorting, I'll have to do it in a new data structure that preserves those indexes

        let mut intervals_sorted_by_start_with_original_index: Vec<(i32, usize)> = intervals
            .iter()
            .enumerate()
            .map(|(index, interval)| (interval[0], index))
            .collect();
        intervals_sorted_by_start_with_original_index.sort_by_key(|(start, _)| *start);

        // println!("intervals: {intervals:?}");

        let mut right_intervals = vec![];

        for interval in intervals.iter() {
            // println!("interval: {interval:?}");
            let mut l = 0;
            let mut r = intervals.len() - 1;
            let target = interval[1];
            // println!("target: {target}");
            while l < r {
                let m: usize = (l + r) / 2;
                if target <= intervals_sorted_by_start_with_original_index[m].0 {
                    r = m;
                } else {
                    l = m + 1;
                }
            }
            // so now intervals[r][0] is the first occurrence of the target, the first thing bigger than it, or it's still smaller but just the biggest thing we could find
            let right_interval = if target <= intervals_sorted_by_start_with_original_index[r].0 {
                intervals_sorted_by_start_with_original_index[r].1 as i32
            } else {
                -1
            };
            right_intervals.push(right_interval);
        }

        right_intervals
    }
}
