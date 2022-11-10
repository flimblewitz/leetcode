fn main() {
    println!(
        "{:?}",
        Solution::merge(vec![vec![1, 3], vec![2, 6], vec![8, 10], vec![15, 18]])
    );
    println!("{:?}", Solution::merge(vec![vec![1, 4], vec![2, 3]]));
    println!(
        "{:?}",
        Solution::merge(vec![
            vec![2, 3],
            vec![4, 5],
            vec![6, 7],
            vec![8, 9],
            vec![1, 10]
        ])
    );
}

struct Solution {}
impl Solution {
    pub fn merge(mut intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        // start looping with the intent to hit every index of intervals
        // remove interval i from the array
        // look for an overlap in the rest of them
        // if you find one, remove that one too, merge them, and put them back in at index i. Then restart the loop
        // if you don't find one, put interval i back in at index i, then increment i and restart the loop
        let mut i = 0;
        while i < intervals.len() {
            let mut interval = intervals.remove(i);
            if let Some((interval_index, _)) =
                intervals.iter().enumerate().find(|(_, other_interval)| {
                    !(other_interval[0] > interval[1] || other_interval[1] < interval[0])
                })
            {
                let other_interval = intervals.remove(interval_index);
                interval = vec![
                    interval[0].min(other_interval[0]),
                    interval[1].max(other_interval[1]),
                ];
                intervals.insert(i, interval);
                continue;
            }
            intervals.insert(i, interval);
            i += 1;
        }
        intervals
    }
}
