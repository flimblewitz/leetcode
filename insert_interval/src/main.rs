fn main() {
    println!("{:?}", Solution::insert(vec![], vec![2, 5]));
    println!(
        "{:?}",
        Solution::insert(vec![vec![1, 3], vec![6, 9]], vec![2, 5])
    );
}

struct Solution {}
impl Solution {
    pub fn insert(intervals: Vec<Vec<i32>>, new_interval: Vec<i32>) -> Vec<Vec<i32>> {
        let mut intervals = intervals;
        if let Some(index_of_first_interval_with_greater_start) = intervals
            .iter()
            .enumerate()
            .find(|(_, interval)| interval[0] > new_interval[0])
            .map(|(index, _)| index)
        {
            intervals.insert(index_of_first_interval_with_greater_start, new_interval)
        } else {
            intervals.push(new_interval);
        }

        intervals = intervals.into_iter().fold(vec![], |mut acc, interval| {
            if let Some(overlapping_interval) = acc.iter_mut().find(|acc_interval| {
                !(acc_interval[0] > interval[1] || acc_interval[1] < interval[0])
            }) {
                *overlapping_interval = vec![
                    overlapping_interval[0].min(interval[0]),
                    overlapping_interval[1].max(interval[1]),
                ]
            } else {
                acc.push(interval);
            }
            acc
        });

        intervals
    }
}
